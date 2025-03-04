use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn pass_kyc_event(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "pass_kyc"), user);
    e.events().publish(topics, true);
}

pub(crate) fn fail_kyc_event(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "fail_kyc"), user);
    e.events().publish(topics, true);
}

pub(crate) fn whitelist_event(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "whitelist"), user);
    e.events().publish(topics, true);
}

pub(crate) fn blacklist_event(e: &Env, user: Address) {
    let topics = (Symbol::new(e, "blacklist"), user);
    e.events().publish(topics, true);
}

pub(crate) fn add_amm_event(e: &Env, address: Address) {
    let topics = (Symbol::new(e, "add_amm"), address);
    e.events().publish(topics, true);
}

pub(crate) fn remove_amm_event(e: &Env, address: Address) {
    let topics = (Symbol::new(e, "remove_amm"), address);
    e.events().publish(topics, true);
}
