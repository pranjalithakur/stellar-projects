use crate::{
    admin::{has_admin, read_admin, write_admin},
    errors::Error,
    ext_token::{read_ext_token, write_ext_token},
    interface::LiquidityPoolTrait,
    loan::{
        has_loan, read_loan, read_rate_percent, write_loan, write_rate_percent, Loan, LoanStatus,
    },
    pool_token::{create_contract, read_pool_token, write_pool_token},
    storage_types::{TokenInfo, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, vec, Address, BytesN, Env, IntoVal, Symbol,
    Val,
};

mod tc_contract {
    soroban_sdk::contractimport!(
        file = "../argentina_pledge/target/wasm32-unknown-unknown/release/argentina_pledge.wasm"
    );
}

#[contract]
pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(
        e: Env,
        admin: Address,
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,
        rate_percent: u32,
    ) {
        if has_admin(&e) {
            panic!("already initialized")
        }
        write_admin(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        // deploy and initialize the token contract
        let pool_token_contract = create_contract(&e, token_wasm_hash);
        e.invoke_contract::<Val>(
            &pool_token_contract,
            &"initialize".into_val(&e),
            vec![
                &e,
                e.current_contract_address().into_val(&e),
                7u32.into_val(&e),
                "Argentina Pool Token".into_val(&e),
                "APT".into_val(&e),
            ],
        );

        write_pool_token(
            &e,
            TokenInfo {
                address: pool_token_contract,
                decimals: 7,
            },
        );
        write_ext_token(
            &e,
            TokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        );
        write_rate_percent(&e, rate_percent);
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&e, &new_admin);
    }

    fn set_rate(e: Env, new_rate: u32) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_rate_percent(&e, new_rate);
    }

    fn deposit(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Transfer USDC from "from" to the contract address
        let ext_token = read_ext_token(&e);
        let client = token::Client::new(&e, &ext_token.address);
        client.transfer(&from, &e.current_contract_address(), &amount);

        // Mint an equal number of liquidity tokens to "from"
        token::StellarAssetClient::new(&e, &read_pool_token(&e).address).mint(&from, &amount);
    }

    fn withdraw(e: Env, from: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Burn the specified number of liquidity tokens from "from"
        token::Client::new(&e, &read_pool_token(&e).address).burn(&from, &amount);

        // Transfer USDC from the contract address to "from"
        token::Client::new(&e, &read_ext_token(&e).address).transfer(
            &e.current_contract_address(),
            &from,
            &amount,
        );
    }

    fn create_loan_offer(e: Env, from: Address, offer_id: i128, tc_address: Address, tc_id: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        if has_loan(&e, offer_id) {
            panic_with_error!(&e, Error::NotEmpty);
        };

        let tc_amount = i128::from(tc_contract::Client::new(&e, &tc_address).get_amount(&tc_id));
        // lock in funds from caller (potential creditor)
        transfer_scaled(&e, from.clone(), e.current_contract_address(), tc_amount, 0);
        let request = Loan {
            id: offer_id,
            borrower: from.clone(),
            creditor: from.clone(),
            amount: i128::from(tc_amount),
            tc_address,
            tc_id,
            rate_percent: read_rate_percent(&e),
            status: LoanStatus::Pending,
        };

        write_loan(&e, request);
    }

    fn cancel_loan_offer(e: Env, offer_id: i128) {
        let mut loan = read_loan(&e, offer_id);
        loan.creditor.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        if loan.status != LoanStatus::Pending {
            panic_with_error!(&e, Error::InvalidStatus);
        }

        // return funds from smart contract to creditor
        transfer_scaled(
            &e,
            e.current_contract_address(),
            loan.creditor.clone(),
            loan.amount,
            0,
        );

        loan.status = LoanStatus::Closed;
        write_loan(&e, loan);
    }

    fn accept_loan_offer(e: Env, from: Address, offer_id: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Pending {
            panic_with_error!(&e, Error::InvalidStatus);
        }

        // transfer the TC from caller (borrower) to creditor
        tc_contract::Client::new(&e, &loan.tc_address).transfer(
            &from.clone(),
            &loan.creditor,
            &loan.tc_id,
        );

        // transfer liquidity tokens from smart contract to caller (borrower)
        transfer_scaled(
            &e,
            e.current_contract_address(),
            from.clone(),
            loan.amount,
            0,
        );

        // update loan info
        loan.borrower = from;
        loan.status = LoanStatus::Active;
        write_loan(&e, loan);
    }

    fn payoff_loan(e: Env, from: Address, offer_id: i128) {
        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Active {
            panic_with_error!(&e, Error::InvalidStatus);
        }
        loan.borrower.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // transfer liquidity tokens from caller (borrower) to smart contract
        // pool rate is the additional percent rate needed to pay off the loan.
        transfer_scaled(
            &e,
            loan.borrower.clone(),
            e.current_contract_address(),
            loan.amount,
            loan.rate_percent,
        );

        // update loan info
        loan.status = LoanStatus::Paid;
        write_loan(&e, loan);
    }

    fn close_loan(e: Env, from: Address, offer_id: i128) {
        let mut loan = read_loan(&e, offer_id);
        if loan.status != LoanStatus::Paid {
            panic_with_error!(&e, Error::InvalidStatus);
        }
        loan.creditor.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // transfer the TC from creditor (caller) to borrower
        tc_contract::Client::new(&e, &loan.tc_address).transfer(
            &loan.creditor.clone(),
            &loan.borrower,
            &loan.tc_id,
        );
        // return funds from smart contract to creditor
        transfer_scaled(
            &e,
            e.current_contract_address(),
            loan.creditor.clone(),
            loan.amount,
            loan.rate_percent,
        );

        // update loan info
        loan.status = LoanStatus::Closed;
        write_loan(&e, loan);
    }

    fn get_loan_rate(e: Env, offer_id: i128) -> u32 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.rate_percent
    }

    fn get_pool_rate(e: Env) -> u32 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_rate_percent(&e)
    }

    fn get_loan_tc(e: Env, offer_id: i128) -> (Address, i128) {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        (loan.tc_address, loan.tc_id)
    }

    fn get_loan_borrower(e: Env, offer_id: i128) -> Address {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.borrower
    }

    fn get_loan_creditor(e: Env, offer_id: i128) -> Address {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.creditor
    }

    fn get_liquidity_token(e: Env) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_pool_token(&e).address
    }

    fn get_ext_token(e: Env) -> (Address, u32) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let ext_token = read_ext_token(&e);
        (ext_token.address, ext_token.decimals)
    }

    fn get_payoff_amount(e: Env, offer_id: i128) -> i128 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let scaled_amount = calculate_scaled_amount_with_interest(
            loan.amount,
            read_pool_token(&e).decimals,
            loan.rate_percent,
        );
        match scaled_amount {
            Some(scaled_amount) => scaled_amount,
            None => panic_with_error!(&e, Error::IntegerOverflow),
        }
    }

    fn get_loan_amount(e: Env, offer_id: i128) -> i128 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.amount
    }

    fn get_loan_status(e: Env, offer_id: i128) -> u32 {
        let loan = read_loan(&e, offer_id);
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        loan.status as u32
    }
}

fn transfer_scaled(e: &Env, from: Address, to: Address, amount: i128, rate: u32) {
    let pool_token = read_pool_token(&e);
    let scaled_amount = calculate_scaled_amount_with_interest(amount, pool_token.decimals, rate);
    match scaled_amount {
        Some(scaled_amount) => {
            token::Client::new(&e, &pool_token.address).transfer(&from, &to, &scaled_amount);
        }
        None => panic_with_error!(&e, Error::IntegerOverflow),
    }
}

fn calculate_scaled_amount_with_interest(amount: i128, decimals: u32, rate: u32) -> Option<i128> {
    if rate == 0 {
        return amount.checked_mul(10i128.pow(decimals));
    }
    amount
        .checked_mul(10i128.pow(decimals))?
        .checked_mul(100 + i128::from(rate))?
        .checked_div(100)
}
