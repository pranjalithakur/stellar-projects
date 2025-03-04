use crate::storage_types::{DataKey, Offer, OFFER_BUMP_AMOUNT, OFFER_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_offer(e: &Env, offer_id: i128) -> Option<Offer> {
    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
        Some(offer)
    } else {
        None
    }
}

pub fn check_offer(e: &Env, offer_id: i128) -> bool {
    let key = DataKey::Offer(offer_id);
    e.storage().persistent().has(&key)
}

pub fn write_offer(
    e: &Env,
    offer_id: i128,
    from: Address,
    pool_token: Address,
    amount: i128,
    tc_contract: Address,
    tc_id: i128,
) {
    let input_offer = Offer {
        from: from,
        pool_token,
        amount: amount,
        tc_contract,
        tc_id,
        status: 0,
    };
    let key = DataKey::Offer(offer_id);
    e.storage().persistent().set(&key, &input_offer);
    e.storage()
        .persistent()
        .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
}

pub fn change_offer(e: &Env, offer_id: i128, status: i128) -> bool {
    let key = DataKey::Offer(offer_id);
    if let Some(offer) = e.storage().persistent().get::<DataKey, Offer>(&key) {
        let mut new_offer = offer;
        new_offer.status = status;
        e.storage().persistent().set(&key, &new_offer);
        e.storage()
            .persistent()
            .extend_ttl(&key, OFFER_LIFETIME_THRESHOLD, OFFER_BUMP_AMOUNT);
        return true;
    } else {
        return false;
    }
}
