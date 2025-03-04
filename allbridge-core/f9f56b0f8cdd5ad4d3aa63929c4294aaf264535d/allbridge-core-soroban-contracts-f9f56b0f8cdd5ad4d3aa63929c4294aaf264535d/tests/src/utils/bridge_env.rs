use ethers_core::types::Signature;
use ethers_signers::LocalWallet;
use shared::{
    consts::CHAIN_ID,
    utils::{hash_message, hash_with_sender},
};
use soroban_sdk::{
    map,
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, U256,
};

use crate::{
    contracts::{
        bridge::{ReceiveFee, Swapped, TokensReceived, TokensSent},
        pool::Pool as PoolInfo,
    },
    utils::{
        assert_rel_eq, consts::BP, float_to_int_sp, get_event_by_name, sign_message,
        MessengerConfig, contract_id
    },
};

use super::{
    consts::{
        GOERLI_CHAIN_ID, GOERLI_GAS_PRICE, GOERLI_PRICE, OTHER_CHAIN_IDS, THIS_GAS_PRICE,
        THIS_PRICE,
    },
    gen_nonce, get_non_compress_public_key, message_hash_vec_to_byte, public_key_to_bytes,
    BalancesSnapshot, Bridge, BridgeEnvConfig, CallResult, GasOracle, Messenger, Pool, Token, User,
};

#[allow(dead_code)]
pub struct BridgeEnv {
    config: BridgeEnvConfig,
    messenger_config: MessengerConfig,

    pub admin: Address,

    pub bridge: Bridge,
    pub gas_oracle: GasOracle,
    pub messenger: Messenger,
    pub native_token: Token,

    pub primary_validator_wallet: LocalWallet,
    pub secondary_validator_wallet: LocalWallet,

    pub alice: User,
    pub bob: User,

    pub yaro_token: Token,
    pub yaro_pool: Pool,
    pub yaro_stop_authority: Address,

    pub yusd_token: Token,
    pub yusd_pool: Pool,
    pub yusd_stop_authority: Address,

    pub goerli_bridge: BytesN<32>,
    pub goerli_token: BytesN<32>,
}

impl BridgeEnv {
    pub fn default(env: &Env) -> BridgeEnv {
        Self::create(env, BridgeEnvConfig::default())
    }

    pub fn override_primary_validator(&mut self, new_primary_validator_pk: &str) {
        self.primary_validator_wallet = new_primary_validator_pk.parse::<LocalWallet>().unwrap()
    }

    pub fn override_secondary_validator(&mut self, new_secondary_validator_pk: &str) {
        self.secondary_validator_wallet = new_secondary_validator_pk.parse::<LocalWallet>().unwrap()
    }

    pub fn create(env: &Env, config: BridgeEnvConfig) -> BridgeEnv {
        env.mock_all_auths();
        env.budget().reset_limits(u64::MAX, u64::MAX);

        let admin = Address::generate(&env);

        let native_token = Token::create(env, "native", &admin);

        let gas_oracle = GasOracle::create(&env, &admin);

        let primary_validator_wallet = config.primary_validator_pk.parse::<LocalWallet>().unwrap();
        let secondary_validator_wallet = config
            .secondary_validator_pk
            .parse::<LocalWallet>()
            .unwrap();

        let primary_validator_public = public_key_to_bytes(
            &env,
            &get_non_compress_public_key(&primary_validator_wallet),
        );
        let secondary_validator_public = public_key_to_bytes(
            &env,
            &get_non_compress_public_key(&secondary_validator_wallet),
        );

        let messenger_config = MessengerConfig {
            chain_id: CHAIN_ID,
            admin: admin.clone(),
            native_token: native_token.id.clone(),
            gas_oracle: gas_oracle.id.clone(),
            other_chain_ids: BytesN::from_array(env, &OTHER_CHAIN_IDS),
            primary_validator_key: primary_validator_public,
            secondary_validator_keys: map![&env, (secondary_validator_public, true)],
        };
        let messenger = Messenger::create(&env, messenger_config.clone());

        let bridge = Bridge::create(
            &env,
            &admin,
            &messenger.id,
            &gas_oracle.id,
            native_token.clone_token(&env),
        );

        let alice = User::generate(env, "alice");
        let bob = User::generate(env, "bob");

        native_token.airdrop_user(&alice);
        native_token.airdrop_user(&bob);
        native_token.airdrop(&bridge.id);

        let (yaro_token, yaro_pool, yaro_stop_authority) = BridgeEnv::create_token_and_pool(
            &env,
            &admin,
            "yaro",
            &bridge,
            config.yaro_fee_share_bp,
            config.yaro_admin_fee,
            config.yaro_admin_deposit,
        );
        yaro_token.airdrop_user(&alice);
        yaro_token.airdrop_user(&bob);

        let (yusd_token, yusd_pool, yusd_stop_authority) = BridgeEnv::create_token_and_pool(
            &env,
            &admin,
            "yusd",
            &bridge,
            config.yusd_fee_share_bp,
            config.yusd_admin_fee,
            config.yusd_admin_deposit,
        );
        yusd_token.airdrop_user(&alice);
        yusd_token.airdrop_user(&bob);

        let (goerli_bridge, goerli_token) =
            bridge.generate_and_register_bridge(&env, GOERLI_CHAIN_ID);

        bridge.generate_and_set_stop_authority(&env);

        gas_oracle.client.set_price(
            &GOERLI_CHAIN_ID,
            &Some(GOERLI_PRICE),
            &Some(GOERLI_GAS_PRICE),
        );
        gas_oracle
            .client
            .set_price(&CHAIN_ID, &Some(THIS_PRICE), &Some(THIS_GAS_PRICE));

        bridge.client.set_gas_usage(&GOERLI_CHAIN_ID, &300_000_000);

        BridgeEnv {
            config,
            messenger_config,

            admin,
            bridge,
            gas_oracle,
            messenger,
            native_token,

            primary_validator_wallet,
            secondary_validator_wallet,

            alice,
            bob,

            yaro_token,
            yaro_pool,
            yaro_stop_authority,

            yusd_token,
            yusd_pool,
            yusd_stop_authority,

            goerli_bridge,
            goerli_token,
        }
    }

    #[inline]
    pub fn native_airdrop(&self, to: &Address) {
        self.native_token.airdrop(&to);
    }

    fn get_token_by_tag(&self, tag: &str) -> &Token {
        match tag {
            "yaro" => &self.yaro_token,
            "yusd" => &self.yusd_token,
            _ => unreachable!("Unexpected token name"),
        }
    }

    fn get_token_by_pool(&self, pool: &Pool) -> &Token {
        if pool.id.eq(&self.yaro_pool.id) {
            &self.yaro_token
        } else {
            &self.yusd_token
        }
    }

    pub fn do_deposit(&self, deposit_amount: f64, user: &User, pool: &Pool) {
        let user_address = user.as_address();
        let token = self.get_token_by_pool(pool);
        let deposit_amount_int = pool.float_to_int(deposit_amount);

        let user_balance_before = token.balance_of(&user_address);
        let user_deposit_before = pool.client.get_user_deposit(&user_address);
        let pool_d_before = pool.d();

        let deposit_amount_sp = token.amount_to_system_precision(deposit_amount_int);

        pool.deposit(&user, deposit_amount).unwrap();

        let user_balance_after = token.balance_of(&user_address);
        let user_deposit_after = pool.client.get_user_deposit(&user_address);
        let pool_d_after = pool.d();

        assert_eq!(user_balance_before - deposit_amount_int, user_balance_after);
        assert_eq!(
            user_deposit_before.lp_amount + deposit_amount_sp,
            user_deposit_after.lp_amount
        );
        assert_eq!(
            user_deposit_after.reward_debt,
            user_deposit_before.reward_debt
        );
        assert_eq!(pool_d_before + deposit_amount_sp, pool_d_after);
    }

    pub fn create_token_and_pool(
        env: &Env,
        admin: &Address,
        token_tag: &'static str,
        bridge: &Bridge,
        fee_share_bp: f64,
        admin_fee: u128,
        admin_deposit: f64,
    ) -> (Token, Pool, Address) {
        let token = Token::create(env, token_tag, &admin);
        let fee_share_bp = ((fee_share_bp as f64) * 10_000.0) as u128;
        let stop_authority = Address::generate(&env);
        let pool = Pool::create(
            env,
            &admin,
            &bridge.id,
            20,
            &token.id,
            fee_share_bp,
            1,
            admin_fee,
        );

        pool.client.set_stop_authority(&stop_authority);

        token.airdrop(&admin);
        if admin_deposit > 0.0 {
            pool.deposit_by_id(&admin, admin_deposit).unwrap();
        }

        bridge.client.add_pool(&pool.id, &token.id);

        (token, pool, stop_authority)
    }

    pub fn hash_and_receive_message(
        &self,
        env: &Env,
        amount_sp: u128,
        recipient: &Address,
        receive_token: &Token,
        nonce: &U256,
    ) -> CallResult<(BytesN<32>, Signature, Signature)> {
        let message_hash = hash_message(
            &env,
            amount_sp,
            &contract_id(&recipient),
            GOERLI_CHAIN_ID,
            CHAIN_ID,
            &contract_id(&receive_token.id),
            nonce,
        );
        let message_hash_with_sender = hash_with_sender(&env, &message_hash, &self.goerli_bridge);
        let message_hash = message_hash_with_sender.to_array().to_vec();

        let primary_signature = sign_message(env, &message_hash, &self.primary_validator_wallet);
        let secondary_signature =
            sign_message(env, &message_hash, &self.secondary_validator_wallet);

        self.messenger
            .receive_message(
                env,
                &message_hash_vec_to_byte(env, &message_hash),
                &primary_signature,
                &secondary_signature,
            )
            .map(|_| {
                (
                    message_hash_with_sender,
                    primary_signature,
                    secondary_signature,
                )
            })
    }

    pub fn do_swap(
        &self,
        env: &Env,
        sender: &User,
        recipient: &User,
        send_token: &Token,
        receive_token: &Token,
        amount: f64,
        receive_amount_threshold: f64,
    ) -> CallResult {
        let amount_int = send_token.float_to_int(amount);
        let receive_amount_min = 0.0f64.max(amount - receive_amount_threshold);

        let receive_amount_min = receive_token.float_to_int(receive_amount_min);

        let before_swap = BalancesSnapshot::take(&self);

        let call_result = self
            .bridge
            .client
            .try_swap(
                &sender.as_address(),
                &amount_int,
                &contract_id(&send_token.id),
                &contract_id(&receive_token.id),
                &recipient.as_address(),
                &receive_amount_min
            )
            .map(Result::unwrap)
            .map_err(Result::unwrap);

        if call_result.is_err() {
            return call_result;
        }

        let after_swap: BalancesSnapshot = BalancesSnapshot::take(&self);
        before_swap.print_change_with(&after_swap, Some("Swap diff"));

        self.assert_swap_results(
            env,
            before_swap,
            after_swap,
            amount,
            amount_int,
            receive_amount_min,
            receive_amount_threshold,
            sender,
            recipient,
            send_token,
            receive_token,
        );

        call_result
    }

    fn assert_swap_to_v_usd(&self, amount_sp: u128, pool_before: &PoolInfo, pool_after: &PoolInfo) {
        let expected_v_usd = pool_before.get_y(pool_after.token_balance);

        assert_eq!(
            pool_before.token_balance + amount_sp,
            pool_after.token_balance
        );
        assert_eq!(pool_before.reserves + amount_sp, pool_after.reserves);
        assert_eq!(pool_after.v_usd_balance, expected_v_usd);
    }

    fn assert_from_v_usd(
        &self,
        pool_before: &PoolInfo,
        pool_after: &PoolInfo,
        amount_sp: u128,
        result_amount_sp: u128,
        receive_amount_min: u128,
        receive_amount_threshold_sp: u128,
    ) {
        assert_eq!(pool_before.d, pool_after.d);
        assert_eq!(
            pool_before.v_usd_balance + amount_sp,
            pool_after.v_usd_balance,
        );

        let new_amount = pool_before.get_y(pool_after.v_usd_balance);
        let calc_result_sp = pool_before.token_balance - new_amount;

        assert_eq!(pool_before.reserves - calc_result_sp, pool_after.reserves);
        assert_eq!(pool_after.token_balance, new_amount);
        assert!(
            result_amount_sp <= amount_sp,
            "result_amount_sp: {}, amount_sp: {}",
            result_amount_sp,
            amount_sp
        );
        assert!(
            result_amount_sp >= receive_amount_min,
            "result_amount_sp: {}, receive_amount_min: {}",
            result_amount_sp,
            receive_amount_min
        );
        assert_rel_eq(
            result_amount_sp,
            receive_amount_min,
            receive_amount_threshold_sp,
        );
    }

    fn assert_swap_results(
        &self,
        env: &Env,
        snapshot_before_swap: BalancesSnapshot,
        snapshot_after_swap: BalancesSnapshot,

        amount: f64,
        amount_int: u128,
        receive_amount_min: u128,
        receive_amount_threshold: f64,

        sender: &User,
        recipient: &User,

        send_token: &Token,
        receive_token: &Token,
    ) {
        let sender_tag = sender.tag;
        let recipient_tag = recipient.tag;
        let send_token_tag = send_token.tag;
        let receive_token_tag = receive_token.tag;

        let sender_send_token_balance_key = format!("{sender_tag}_{send_token_tag}_balance");
        let pool_send_token_balance_key = format!("pool_{send_token_tag}_balance");
        let recipient_receive_token_balance_key =
            format!("{recipient_tag}_{receive_token_tag}_balance");
        let pool_receive_token_balance_key = format!("pool_{receive_token_tag}_balance");

        let swapped_event = get_event_by_name::<Swapped>(env, "Swapped").unwrap();

        let send_pool_before = snapshot_before_swap.get_pool_info_by_tag(send_token_tag);
        let send_pool_after = snapshot_after_swap.get_pool_info_by_tag(send_token_tag);

        let receive_pool_before = snapshot_before_swap.get_pool_info_by_tag(receive_token_tag);
        let receive_pool_after = snapshot_after_swap.get_pool_info_by_tag(receive_token_tag);

        assert_eq!(send_pool_before.d, send_pool_after.d);
        assert_eq!(receive_pool_before.d, receive_pool_after.d);

        let fee = (amount_int * send_pool_before.fee_share_bp) / BP as u128;

        let amount_sp = send_token.amount_to_system_precision(amount_int - fee);

        self.assert_swap_to_v_usd(amount_sp, &send_pool_before, &send_pool_after);

        let receive_amount_min_sp = float_to_int_sp(amount - receive_amount_threshold);
        let receive_amount_threshold = float_to_int_sp(receive_amount_threshold);

        self.assert_from_v_usd(
            &receive_pool_before,
            &receive_pool_after,
            amount_sp,
            receive_token.amount_to_system_precision(swapped_event.receive_amount),
            receive_amount_min_sp,
            receive_amount_threshold,
        );

        assert_eq!(
            snapshot_before_swap[sender_send_token_balance_key.as_str()] - amount_int,
            snapshot_after_swap[sender_send_token_balance_key.as_str()]
        );
        assert!(
            snapshot_after_swap[recipient_receive_token_balance_key.as_str()]
                >= snapshot_before_swap[recipient_receive_token_balance_key.as_str()]
                    + receive_amount_min
        );
        assert_eq!(
            snapshot_before_swap[pool_send_token_balance_key.as_str()] + amount_int,
            snapshot_after_swap[pool_send_token_balance_key.as_str()],
        );
        assert_eq!(
            snapshot_before_swap[pool_receive_token_balance_key.as_str()] - amount_int,
            snapshot_after_swap[pool_receive_token_balance_key.as_str()],
        );

        assert_eq!(swapped_event.send_amount, amount_int);
        assert!(swapped_event.receive_amount >= receive_amount_min);

        assert_eq!(swapped_event.sender, sender.as_address());
        assert_eq!(swapped_event.recipient, recipient.as_address());

        assert_eq!(swapped_event.receive_token, contract_id(&receive_token.id));
        assert_eq!(swapped_event.send_token, contract_id(&send_token.id));
    }

    pub fn do_swap_and_bridge(
        &self,
        env: &Env,
        sender: &User,
        token: &Token,
        amount: f64,
        gas_amount: f64,
        fee_token_amount: f64,
    ) -> CallResult {
        let nonce = gen_nonce(&env);
        let recipient = BytesN::random(&env);

        let snapshot_before_swap = BalancesSnapshot::take(&self);

        let call_result = self.bridge.swap_and_bridge(
            sender,
            token,
            amount,
            gas_amount,
            fee_token_amount,
            GOERLI_CHAIN_ID,
            &recipient,
            &self.goerli_token,
            &nonce,
        );

        if call_result.is_err() {
            return call_result;
        }

        let snapshot_after_swap = BalancesSnapshot::take(&self);
        snapshot_before_swap.print_change_with(&snapshot_after_swap, Some("SwapAndBridge diff"));

        let amount_int = token.float_to_int(amount);
        let gas_amount_int = self.native_token.float_to_int(gas_amount);
        let fee_token_amount_int = token.float_to_int(fee_token_amount);

        self.assert_swap_and_bridge_results(
            env,
            snapshot_before_swap,
            snapshot_after_swap,
            amount_int,
            gas_amount_int,
            fee_token_amount_int,
            &nonce,
            &recipient,
            &sender,
            &token,
        );

        call_result
    }

    fn assert_swap_and_bridge_results(
        &self,
        env: &Env,
        snapshot_before_swap: BalancesSnapshot,
        snapshot_after_swap: BalancesSnapshot,

        amount: u128,
        gas_amount: u128,
        fee_token_amount: u128,

        nonce: &U256,
        recipient: &BytesN<32>,

        user: &User,
        token: &Token,
    ) {
        let token_tag = token.tag;
        let user_tag = user.tag;

        let pool_balance_key = format!("pool_{token_tag}_balance");
        let user_token_balance_key = format!("{user_tag}_{token_tag}_balance");
        let user_native_balance_key = format!("{user_tag}_native_balance");
        let bridge_token_balance_key = format!("bridge_{token_tag}_balance");

        let receive_fee = get_event_by_name::<ReceiveFee>(&env, "ReceiveFee").unwrap();
        let tokens_sent_event = get_event_by_name::<TokensSent>(&env, "TokensSent").unwrap();

        let bridge_tx_cost = self.bridge.client.get_transaction_cost(&GOERLI_CHAIN_ID);
        let messenger_tx_cost = self.messenger.client.get_transaction_cost(&GOERLI_CHAIN_ID);

        let pool_before = snapshot_before_swap.get_pool_info_by_tag(token_tag);
        let pool_after = snapshot_after_swap.get_pool_info_by_tag(token_tag);

        let amount_after_token_fee = amount - fee_token_amount;
        let fee = (amount_after_token_fee * pool_before.fee_share_bp) / BP as u128;
        let amount_sp = token.amount_to_system_precision(amount_after_token_fee - fee);

        self.assert_swap_to_v_usd(amount_sp, &pool_before, &pool_after);

        assert_eq!(bridge_tx_cost, receive_fee.bridge_transaction_cost);
        assert_eq!(messenger_tx_cost, receive_fee.message_transaction_cost);

        assert_eq!(nonce.clone(), tokens_sent_event.nonce);
        assert_eq!(recipient.clone(), tokens_sent_event.recipient);

        assert_eq!(GOERLI_CHAIN_ID, tokens_sent_event.destination_chain_id);
        assert_eq!(self.goerli_token, tokens_sent_event.receive_token);

        assert_eq!(
            snapshot_before_swap.bridge_native_balance + gas_amount - messenger_tx_cost,
            snapshot_after_swap.bridge_native_balance,
        );
        assert_eq!(
            snapshot_before_swap[pool_balance_key.as_str()] + (amount - fee_token_amount),
            snapshot_after_swap[pool_balance_key.as_str()],
        );
        assert_eq!(
            snapshot_before_swap.messenger_native_balance + messenger_tx_cost,
            snapshot_after_swap.messenger_native_balance,
        );
        assert_eq!(
            snapshot_before_swap[user_token_balance_key.as_str()] - amount,
            snapshot_after_swap[user_token_balance_key.as_str()]
        );
        assert_eq!(
            snapshot_before_swap[user_native_balance_key.as_str()] - gas_amount,
            snapshot_after_swap[user_native_balance_key.as_str()]
        );
        assert_eq!(
            snapshot_before_swap[bridge_token_balance_key.as_str()] + fee_token_amount,
            snapshot_after_swap[bridge_token_balance_key.as_str()]
        );

        let expected_max_token_sent_amount_sp =
            token.amount_to_system_precision(amount - fee_token_amount);

        println!(
            "expected_max_token_sent_amount_sp {:?}",
            expected_max_token_sent_amount_sp
        );
        println!("tokens_sent_event.amount {:?}", tokens_sent_event.amount);
    }

    pub fn do_receive_tokens(
        &self,
        env: &Env,
        amount: f64,
        extra_gas: u128,
        receive_amount_threshold: f64,

        user: &User,
        token: &Token,
    ) -> CallResult {
        let nonce = gen_nonce(env);
        let amount_sp = float_to_int_sp(amount);
        let receive_amount_min = 0.0f64.max(amount - receive_amount_threshold);
        let snapshot_before_swap = BalancesSnapshot::take(&self);

        let (message_hash_with_sender, _, _) = self
            .hash_and_receive_message(&env, amount_sp, &user.as_address(), &token, &nonce)
            .unwrap();

        let call_result = self.bridge.receive_tokens(
            &self.bridge.id,
            amount,
            &user,
            GOERLI_CHAIN_ID,
            &token,
            &nonce,
            receive_amount_min,
            false,
            &Some(extra_gas),
        );

        if call_result.is_err() {
            return call_result;
        }

        let snapshot_after_swap = BalancesSnapshot::take(&self);
        snapshot_before_swap.print_change_with(&snapshot_after_swap, Some("ReceiveTokens diff"));

        let receive_amount_min_sp = float_to_int_sp(amount - receive_amount_threshold);
        self.assert_receive_tokens_result(
            &env,
            snapshot_before_swap,
            snapshot_after_swap,
            amount_sp,
            extra_gas,
            receive_amount_min_sp,
            receive_amount_threshold,
            message_hash_with_sender,
            nonce,
            user,
            token,
        );

        call_result
    }

    fn assert_receive_tokens_result(
        &self,
        env: &Env,
        snapshot_before_swap: BalancesSnapshot,
        snapshot_after_swap: BalancesSnapshot,

        amount_sp: u128,
        extra_gas: u128,
        receive_amount_min: u128,
        receive_amount_threshold: f64,
        message_hash_with_sender: BytesN<32>,
        nonce: U256,

        user: &User,
        token: &Token,
    ) {
        let user_tag = user.tag;
        let token_tag = token.tag;

        let pool_balance_key = format!("pool_{token_tag}_balance");
        let user_token_balance_key = format!("{user_tag}_{token_tag}_balance");
        let user_native_balance_key = format!("{user_tag}_native_balance");

        let receive_amount_threshold_sp = float_to_int_sp(receive_amount_threshold);

        let tokens_received_event =
            get_event_by_name::<TokensReceived>(&env, "TokensReceived").unwrap();

        let result_amount_sp = self
            .get_token_by_tag(token_tag)
            .amount_to_system_precision(tokens_received_event.amount);

        let pool_before = snapshot_before_swap.get_pool_info_by_tag(token_tag);
        let pool_after = snapshot_after_swap.get_pool_info_by_tag(token_tag);

        self.assert_from_v_usd(
            &pool_before,
            &pool_after,
            amount_sp,
            result_amount_sp,
            receive_amount_min,
            receive_amount_threshold_sp,
        );

        assert_eq!(
            snapshot_before_swap[user_native_balance_key.as_str()] + extra_gas,
            snapshot_after_swap[user_native_balance_key.as_str()]
        );

        assert_eq!(
            snapshot_before_swap.bridge_native_balance - extra_gas,
            snapshot_after_swap.bridge_native_balance
        );

        assert_eq!(
            snapshot_before_swap[user_token_balance_key.as_str()] + tokens_received_event.amount,
            snapshot_after_swap[user_token_balance_key.as_str()]
        );

        assert_eq!(
            snapshot_before_swap[pool_balance_key.as_str()] - tokens_received_event.amount,
            snapshot_after_swap[pool_balance_key.as_str()]
        );

        assert_eq!(tokens_received_event.message, message_hash_with_sender);
        assert_eq!(tokens_received_event.recipient, user.contract_id());
        assert_eq!(tokens_received_event.nonce, nonce);
    }
}
