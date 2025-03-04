use crate::{
    errors::Error,
    storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD},
};
use soroban_sdk::{contracttype, panic_with_error, Address, Env};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum LoanStatus {
    Pending = 0,
    Active = 1,
    Paid = 2,
    Closed = 3,
}

#[derive(Clone)]
#[contracttype]
pub struct Loan {
    pub id: i128,
    pub borrower: Address,
    pub creditor: Address,
    pub amount: i128,
    pub tc_address: Address,
    pub tc_id: i128,
    pub rate_percent: u32,
    pub status: LoanStatus,
}

pub fn write_rate_percent(e: &Env, rate_percent: u32) {
    let key = DataKey::RatePercent;
    e.storage().instance().set(&key, &rate_percent);
}

pub fn read_rate_percent(e: &Env) -> u32 {
    let key = DataKey::RatePercent;
    match e.storage().instance().get::<DataKey, u32>(&key) {
        Some(rate_percent) => rate_percent,
        None => 0,
    }
}

pub fn write_loan(e: &Env, loan: Loan) {
    let key = DataKey::Loan(loan.id);
    e.storage().persistent().set(&key, &loan);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn has_loan(e: &Env, offer_id: i128) -> bool {
    let key = DataKey::Loan(offer_id);
    e.storage().persistent().has(&key)
}

pub fn read_loan(e: &Env, offer_id: i128) -> Loan {
    let key = DataKey::Loan(offer_id);
    match e.storage().persistent().get(&key) {
        Some(data) => {
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            data
        }
        None => panic_with_error!(&e, Error::NotFound),
    }
}
