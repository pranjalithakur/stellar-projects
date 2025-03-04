use soroban_sdk::{Address, BytesN, Env, String};

pub trait LiquidityPoolTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract with "admin" as administrator.
    /// token_wasm_hash is for automatically deploying a liquidity token contract.
    /// ext_token_* parameters are for the wrapped USDC contract.
    /// rate is the default rate for paying off loans, expressed as a percentage.
    fn initialize(
        e: Env,
        admin: Address,
        token_wasm_hash: BytesN<32>,
        ext_token_address: Address,
        ext_token_decimals: u32,
        rate_percent: u32,
    );

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(e: Env, new_admin: Address);

    /// Set the liquidity pool's return rate.
    fn set_rate(e: Env, new_rate: u32);

    // --------------------------------------------------------------------------------
    // Pool interface
    // --------------------------------------------------------------------------------

    /// Deposit USDC into the contract in exchange for a corresponding number of liquidity tokens minted to the "from" address.
    /// Emit event with topics = ["deposit", from: Address], data = [amount: u32]
    fn deposit(e: Env, from: Address, amount: i128);

    /// Withdraw USDC from the contract in exchange for a corresponding number of liquidity tokens burned from the "from" address.
    /// Emit event with topics = ["withdraw", from: Address], data = [amount: u32]
    fn withdraw(e: Env, from: Address, amount: i128);

    /// Create a loan offer against a TC. The caller (creditor) transfers liquidity tokens to the smart contract equal to the value of the TC.
    /// The loan will use the liquidity pool's interest rate at the time of the offer being created
    fn create_loan_offer(e: Env, from: Address, offer_id: i128, tc_addr: Address, tc_id: i128);

    /// Cancel a loan offer. Caller must be the user who created the request (creditor).
    /// Transfers the liquidity tokens back to the caller.
    fn cancel_loan_offer(e: Env, offer_id: i128);

    /// Accept a loan offer. The caller (borrower) must own the TC or have approval to transfer it.
    /// Transfers the TC to the creditor, and liquidity tokens equal to the associated TC's value are sent from the smart contract to the caller.
    fn accept_loan_offer(e: Env, from: Address, offer_id: i128);

    /// Pay off a loan. The caller (borrower) transfers liquidity tokens to the smart contract.
    /// If the pool's rate is greater than 0, the amount of liquidity tokens required to pay off is higher than the original amount.
    /// The loan offer must be accepted prior to this step.
    fn payoff_loan(e: Env, from: Address, offer_id: i128);

    /// Close a loan by returning the TC from the creditor to the borrower, then sending the liquidity tokens from the smart contract back to the creditor.
    /// Payoff must be completed prior to this step.
    fn close_loan(e: Env, from: Address, offer_id: i128);

    /// Get the rate associated with a loan.
    fn get_loan_rate(e: Env, offer_id: i128) -> u32;

    /// Get the liquidity pool's current rate.
    fn get_pool_rate(e: Env) -> u32;

    /// Get the contract address and TC id associated with a loan.
    fn get_loan_tc(e: Env, offer_id: i128) -> (Address, i128);

    /// Get the borrower associated with a loan.
    fn get_loan_borrower(e: Env, offer_id: i128) -> Address;

    /// Get the creditor associated with a loan.
    fn get_loan_creditor(e: Env, offer_id: i128) -> Address;

    /// Get the contract address of the liquidity pool token.
    fn get_liquidity_token(e: Env) -> Address;

    /// Get the contract address and decimals of the USDC contract.
    fn get_ext_token(e: Env) -> (Address, u32);

    /// Get the amount required to successfully pay off the loan.
    fn get_payoff_amount(e: Env, offer_id: i128) -> i128;

    /// Get the base amount of the loan
    fn get_loan_amount(e: Env, offer_id: i128) -> i128;

    /// Get the status of a loan
    fn get_loan_status(e: Env, offer_id: i128) -> u32;
}
