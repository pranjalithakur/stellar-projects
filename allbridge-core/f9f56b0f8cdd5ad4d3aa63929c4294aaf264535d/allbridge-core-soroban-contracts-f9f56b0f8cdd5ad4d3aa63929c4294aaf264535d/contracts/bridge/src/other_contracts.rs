#![allow(clippy::too_many_arguments)]

pub mod gas_oracle {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/gas_oracle.wasm"
    );
}

pub mod messenger {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/messenger.wasm"
    );
}

pub mod pool {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/pool.wasm");
}
