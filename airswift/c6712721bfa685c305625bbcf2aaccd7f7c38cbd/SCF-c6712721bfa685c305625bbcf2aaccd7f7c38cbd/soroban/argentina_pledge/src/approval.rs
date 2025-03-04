use crate::errors::Error;
use crate::storage_types::DataKey;
use crate::storage_types::{
    ApprovalAll, ApprovalKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_approval(e: &Env, id: i128) -> Address {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    if let Some(approval) = e.storage().persistent().get::<DataKey, Address>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        approval
    } else {
        panic_with_error!(e, Error::NotAuthorized)
    }
}

pub fn read_approval_all(e: &Env, owner: Address, operator: Address) -> bool {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    if let Some(approval) = e.storage().persistent().get::<DataKey, bool>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        approval
    } else {
        false
    }
}

pub fn write_approval(e: &Env, id: i128, operator: Option<Address>) {
    let key = DataKey::Approval(ApprovalKey::ID(id));
    e.storage().persistent().set(&key, &operator);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn write_approval_all(e: &Env, owner: Address, operator: Address, approved: bool) {
    let key = DataKey::Approval(ApprovalKey::All(ApprovalAll { operator, owner }));
    e.storage().persistent().set(&key, &approved);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
