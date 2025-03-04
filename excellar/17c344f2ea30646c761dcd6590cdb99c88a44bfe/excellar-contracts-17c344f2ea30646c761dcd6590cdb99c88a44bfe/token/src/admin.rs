use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn write_kyc(e: &Env, addr: Address) {
    let key = DataKey::Kyc(addr);
    e.storage().instance().set(&key, &true);
}

pub fn remove_kyc(e: &Env, addr: Address) {
    let key = DataKey::Kyc(addr);
    e.storage().instance().remove(&key);
}

pub fn is_kyc_passed(e: &Env, addr: Address) -> bool {
    let key = DataKey::Kyc(addr);
    e.storage().instance().has(&key)
}

pub fn check_kyc_passed(e: &Env, addr: Address) {
    let passed = is_kyc_passed(e, addr);
    if !passed {
        panic!("address is not passed kyc");
    }
}

pub fn remove_blacklist(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    e.storage().instance().remove(&key);
}

pub fn write_blacklist(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    e.storage().instance().set(&key, &true);
}

pub fn check_not_blacklisted(e: &Env, addr: Address) {
    let key = DataKey::Blacklisted(addr);
    let res = e.storage().instance().has(&key);
    if res {
        panic!("address is blacklisted");
    }
}

pub fn remove_amm(e: &Env, addr: Address) {
    let key = DataKey::Amm(addr);
    e.storage().instance().remove(&key);
}

pub fn add_amm(e: &Env, addr: Address) {
    let key = DataKey::Amm(addr);
    e.storage().instance().set(&key, &true);
}

pub fn is_amm(e: &Env, addr: Address) -> bool {
    let key = DataKey::Amm(addr);
    e.storage().instance().has(&key)
}

pub fn check_not_amm(e: &Env, addr: Address) {
    if is_amm(e, addr.clone()) {
        panic!("amm address not allowed")
    }
}
pub fn require_admin(e: &Env) -> Address {
    let admin = read_administrator(e);
    admin.require_auth();
    admin
}
