use soroban_sdk::{contracttype, symbol_short, unwrap::UnwrapOptimized, Address, Env, Symbol};

const ORDERINFO_KEY: Symbol = symbol_short!("ORDERINFO");

#[contracttype]
#[derive(Clone)]
pub struct TokenOrderInfo {
    pub buyer_address: Address,
    pub total_amount: u32,
    pub end_time: u64,
}

pub struct OrderInfoUtils {
    env: Env,
}

impl OrderInfoUtils {
    pub fn new(env: &Env) -> OrderInfoUtils {
        OrderInfoUtils { env: env.clone() }
    }

    #[inline(always)]
    pub fn set_order_info(&self, order_info: &TokenOrderInfo) {
        self.env
            .storage()
            .instance()
            .set(&ORDERINFO_KEY, order_info);
    }

    #[inline(always)]
    pub fn get_order_info(&self) -> TokenOrderInfo {
        self.env
            .storage()
            .instance()
            .get(&ORDERINFO_KEY)
            .unwrap_optimized()
    }
}

pub fn write_order_info(env: &Env, buyer_address: Address, total_amount: u32, end_time: u64) {
    let util = OrderInfoUtils::new(env);
    let order_info = TokenOrderInfo {
        buyer_address,
        total_amount,
        end_time,
    };
    util.set_order_info(&order_info);
}

pub fn read_total_amount(env: &Env) -> u32 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().total_amount
}

pub fn read_end_time(env: &Env) -> u64 {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().end_time
}

pub fn read_buyer_address(env: &Env) -> Address {
    let util = OrderInfoUtils::new(env);
    util.get_order_info().buyer_address
}
