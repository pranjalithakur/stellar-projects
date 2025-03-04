#!/bin/bash
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
cd $SCRIPTPATH
soroban contract extend --id $(cat ./contract_address) --network $(cat ../network_name) --source-account admin --durability persistent --ledgers-to-extend 3000000
soroban contract extend --wasm target/wasm32-unknown-unknown/release/contract_deployer.wasm --network $(cat ../network_name) --source-account admin --durability persistent --ledgers-to-extend 3000000
