# SCF

## Environment
Follow the steps according to https://soroban.stellar.org/docs/getting-started/setup 
* For convenience, the repository contains scripts for deploying and bumping smart contracts. Specify the network name in the `network_name` file, the contents of which will be passed to any `--network` argument in the deploy and bump scripts.
* To automatically bump the contracts, you can call the scripts at regular intervals via crontab. Once per month is enough on official Stellar mainnet/testnet/futurenet. For networks running on local Stellar quickstart images, it may be better to bump once per week due to the quicker ledger time.

## Build
1. ```bash
   cd soroban
   ```
2. ```bash
   make
   ```

## Run Unit Tests
1. `cd` to each desired sub-directory of the `soroban` folder, and run `cargo test`

## Setup identities on Soroban CLI
1. ```bash
   soroban config identity generate --global admin && \
   soroban config identity generate --global acc1 && \
   soroban config identity address admin && \
   soroban config identity address acc1
   ```
2. The addresses for the accounts will be output. Take note of the addresses which will be used to initialize the contract and mint the tokens.
3. If using these accounts on a non-sandbox environment, you will have to fund them with Friendbot first.

## Deploy contracts
These steps assume that you have a network for your CLI set up already. If not, please refer to https://soroban.stellar.org/docs/getting-started/deploy-to-futurenet#configure-futurenet-in-your-cli
1. `cd` to the contract's subdirectory.
   ```bash
   cd soroban/contract_deployer
   ```
2. Deploy the contract.
   ```bash
   sh deploy.sh
   ```
3. Bump the contract instance and wasm code lifetime.
   ```bash
   sh bump.sh
   ```

## Prevent contract expiration 
1. To keep the contract from expiring after some time, you may want to set up a cron job that runs the same command from step 3 on a monthly basis.
2. Edit your crontab configuration using `crontab -e`. Add lines such as the following to your crontab, replacing the path to the bump.sh files with your own. These commands run bump.sh daily.
   ```
   0 0 * * * /home/ubuntu/scf/soroban/contract_deployer/bump.sh
   0 0 * * * /home/ubuntu/scf/soroban/scf_pool/bump.sh
   0 0 * * * /home/ubuntu/scf/soroban/scf_soroban/bump.sh
   ```
