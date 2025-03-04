# Contract Deployer

This contract deployer can be used to atomically deploy a contract and call functions to initialize the deployed contract. It takes in a list of function names and a list of function arguments, so it can call multiple initialization functions if needed. For example, this is used to deploy the scf_soroban smart contract, then call `initialize` and `set_external_token_provider`.
