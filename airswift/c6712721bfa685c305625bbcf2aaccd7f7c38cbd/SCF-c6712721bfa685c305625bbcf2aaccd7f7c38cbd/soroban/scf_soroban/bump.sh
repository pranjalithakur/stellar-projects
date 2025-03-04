#!/bin/bash
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
cd $SCRIPTPATH
soroban contract extend --wasm target/wasm32-unknown-unknown/release/scf_soroban.wasm --network $(cat ../network_name) --source-account admin --durability persistent --ledgers-to-extend 3000000
