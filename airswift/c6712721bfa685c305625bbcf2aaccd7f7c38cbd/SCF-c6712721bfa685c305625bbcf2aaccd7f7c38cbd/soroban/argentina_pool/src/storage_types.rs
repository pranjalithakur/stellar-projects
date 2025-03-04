use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct TokenInfo {
    pub address: Address,
    pub decimals: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    ExtToken,
    PoolToken,
    Supply,
    Amount(i128),
    RedeemTime(i128),
    Owner(i128),
    RatePercent,
    Loan(i128),
}
