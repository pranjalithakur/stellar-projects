#!/bin/bash
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
cd $SCRIPTPATH
soroban contract deploy --network $(cat ../network_name) --source admin --wasm target/wasm32-unknown-unknown/release/argentina_pool.wasm | tee ./contract_address
