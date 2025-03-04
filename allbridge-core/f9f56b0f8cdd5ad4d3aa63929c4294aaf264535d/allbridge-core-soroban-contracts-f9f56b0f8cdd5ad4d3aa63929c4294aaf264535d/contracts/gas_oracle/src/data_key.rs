use soroban_sdk::contracttype;

pub const PRICE_BUMP_AMOUNT: u32 = 34560; // 2 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ChainData(u32),
    Admin,
}
