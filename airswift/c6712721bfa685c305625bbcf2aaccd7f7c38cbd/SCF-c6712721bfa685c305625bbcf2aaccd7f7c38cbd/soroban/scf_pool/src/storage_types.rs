use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const OFFER_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const OFFER_LIFETIME_THRESHOLD: u32 = OFFER_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct Offer {
    pub from: Address,
    pub pool_token: Address,
    pub amount: i128,
    pub tc_contract: Address,
    pub tc_id: i128,
    pub status: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer(i128),
    PoolTokenWasmHash,
    PoolTokens,        // contains a map of pool token address -> asset token address
    ExtToken(Address), // maps asset token address -> pool token address
    Admin,
}
