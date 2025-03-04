use soroban_sdk::{Address, Env};

use crate::contracts::gas_oracle;

pub struct GasOracle {
    pub id: soroban_sdk::Address,
    pub client: gas_oracle::Client<'static>,
}

impl GasOracle {
    pub fn create(env: &Env, admin: &Address) -> GasOracle {
        let id = env.register_contract_wasm(None, gas_oracle::WASM);
        let client = gas_oracle::Client::new(&env, &id);

        client.initialize(admin);

        GasOracle { id, client }
    }
}
