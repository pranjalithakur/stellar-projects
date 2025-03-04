use std::cmp::Ordering;

use soroban_sdk::{token, Address, Env};

use crate::utils::{consts::SP, float_to_int, int_to_float};

use super::User;

pub struct Token {
    pub id: soroban_sdk::Address,
    pub tag: &'static str,
    pub client: token::Client<'static>,
    pub asset_client: token::StellarAssetClient<'static>,
}

impl Token {
    pub fn create(env: &Env, tag: &'static str, admin: &Address) -> Token {
        let id = env.register_stellar_asset_contract(admin.clone());
        let client = token::Client::new(&env, &id);
        let asset_client = token::StellarAssetClient::new(&env, &id);

        Token {
            id,
            tag,
            client,
            asset_client,
        }
    }

    pub fn clone_token(&self, env: &Env) -> Token {
        let client = token::Client::new(env, &self.id);
        let asset_client = token::StellarAssetClient::new(&env, &self.id);

        Token {
            id: self.id.clone(),
            tag: self.tag,
            client,
            asset_client,
        }
    }

    pub fn airdrop(&self, id: &Address) {
        self.asset_client
            .mint(id, &(self.float_to_int(1_000_000_000.0) as i128));
    }

    pub fn airdrop_user(&self, user: &User) {
        self.airdrop(&user.as_address())
    }

    pub fn amount_to_system_precision(&self, amount: u128) -> u128 {
        let decimals = self.client.decimals();

        match decimals.cmp(&SP) {
            Ordering::Greater => amount / (10u128.pow(decimals - SP)),
            Ordering::Less => amount * (10u128.pow(SP - decimals)),
            Ordering::Equal => amount,
        }
    }

    pub fn amount_from_system_precision(&self, amount: u128) -> u128 {
        let decimals = self.client.decimals();

        match decimals.cmp(&SP) {
            Ordering::Greater => amount * (10u128.pow(decimals - SP)),
            Ordering::Less => amount / (10u128.pow(SP - decimals)),
            Ordering::Equal => amount,
        }
    }

    pub fn balance_of(&self, id: &Address) -> u128 {
        self.client.balance(&id) as u128
    }

    pub fn float_to_int(&self, amount: f64) -> u128 {
        float_to_int(amount, self.client.decimals())
    }

    pub fn int_to_float(&self, amount: u128) -> f64 {
        int_to_float(amount, self.client.decimals())
    }
}
