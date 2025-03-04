.DEFAULT_GOAL := all

all: build-bridge

optimize-all: optimize-gas-oracle optimize-messenger optimize-pool optimize-bridge

#NATIVE_ADDRESS = CB64D3G7SM2RTH6JSGG34DDTFTQ5CFDKVDZJZSODMCX4NJ2HV2KN7OHT #Futurenet
NATIVE_ADDRESS = CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC #Testnet
MESSENGER_ADDRESS_PATH = soroban-deploy/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

GAS_ORACLE_ADDRESS_PATH = soroban-deploy/gas_orace
GAS_ORACLE_WASM_PATH = target/wasm32-unknown-unknown/release/gas_oracle.wasm
GAS_ORACLE_WASM_PATH_OP = target/wasm32-unknown-unknown/release/gas_oracle.optimized.wasm
GAS_ORACLE_ADDRESS = $$(cat $(GAS_ORACLE_ADDRESS_PATH))

POOL_WASM_PATH = target/wasm32-unknown-unknown/release/pool.wasm
POOL_WASM_PATH_OP = target/wasm32-unknown-unknown/release/pool.optimized.wasm
POOL_YARO_ADDRESS_PATH = soroban-deploy/pool_yaro
POOL_YARO_ADDRESS = $$(cat $(POOL_YARO_ADDRESS_PATH))

POOL_USDY_ADDRESS_PATH = soroban-deploy/pool_usdy
POOL_USDY_ADDRESS = $$(cat $(POOL_USDY_ADDRESS_PATH))

MESSENGER_WASM_PATH = target/wasm32-unknown-unknown/release/messenger.wasm
MESSENGER_WASM_PATH_OP = target/wasm32-unknown-unknown/release/messenger.optimized.wasm
MESSENGER_ADDRESS_PATH = soroban-deploy/messenger
MESSENGER_ADDRESS = $$(cat $(MESSENGER_ADDRESS_PATH))

BRIDGE_WASM_PATH = target/wasm32-unknown-unknown/release/bridge.wasm
BRIDGE_WASM_PATH_OP = target/wasm32-unknown-unknown/release/bridge.optimized.wasm
BRIDGE_ADDRESS_PATH = soroban-deploy/bridge
BRIDGE_ADDRESS = $$(cat $(BRIDGE_ADDRESS_PATH))

POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_YARO_ADDRESS)

ALICE = $$(soroban config identity address alice)
ADMIN_ALIAS = alice
ADMIN = $$(soroban config identity address $(ADMIN_ALIAS))

#YARO_ADDRESS=CDFVZVTV4K5S66GQXER7YVK6RB23BMPMD3HQUA3TGEZUGDL3NM3R5GDW #Futurenet
#USDY_ADDRESS=CD7KQQY27G5WXQT2IUYJVHNQH6N2I6GEM5ND2BLZ2GHDAPB2V3KWCW7M #Futurenet

YARO_ADDRESS=CACOK7HB7D7SRPMH3LYYOW77T6D4D2F7TR7UEVKY2TVSUDSRDM6DZVLK #Testnet
USDY_ADDRESS=CAOPX7DVI3PFLHE7637YSFU6TLG6Z27Z5O3M547ANAYXQOAYCYYV6NO6 #Testnet

#TOKEN_ADDRESS=$(YARO_ADDRESS)
#POOL_ADDRESS_PATH=$(POOL_YARO_ADDRESS_PATH)
#POOL_ADDRESS=$(POOL_YARO_ADDRESS)

TOKEN_ADDRESS=$(USDY_ADDRESS)
POOL_ADDRESS_PATH=$(POOL_USDY_ADDRESS_PATH)
POOL_ADDRESS=$(POOL_USDY_ADDRESS)

NETWORK=testnet

test: all
	cargo test

build-gas-oracle:
	 cargo build --target wasm32-unknown-unknown --release --package gas-oracle

build-messenger: build-gas-oracle
	 cargo build --target wasm32-unknown-unknown --release --package messenger

build-pool: 
	cargo build --target wasm32-unknown-unknown --release --package pool

build-bridge: build-messenger build-pool
	cargo build --target wasm32-unknown-unknown --release --package bridge

optimize-gas-oracle:
	soroban contract optimize --wasm $(GAS_ORACLE_WASM_PATH)

optimize-messenger:
	soroban contract optimize --wasm $(MESSENGER_WASM_PATH)

optimize-pool:
	soroban contract optimize --wasm $(POOL_WASM_PATH)

optimize-bridge:
	soroban contract optimize --wasm $(BRIDGE_WASM_PATH)

deploy-gas-oracle:
	soroban contract deploy \
      --wasm $(GAS_ORACLE_WASM_PATH) \
      --source $(ADMIN_ALIAS) \
      --network $(NETWORK) 	\
      > $(GAS_ORACLE_ADDRESS_PATH) && echo $(GAS_ORACLE_ADDRESS)

gas-oracle-init:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN)

gas-oracle-set-price:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 7 \
        --price 1000000000000000000 \
        --gas_price 50

gas-oracle-set-price-1:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_price \
		--chain_id 1 \
        --price 1000000000000000000 \
        --gas_price 50

gas-oracle-get-price-data:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_price \
		--chain_id 1

gas-oracle-get-price:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_price \
		--chain_id 1

gas-oracle-get_admin:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_admin

gas-oracle-get-gas-cost-in-native-token:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_cost_in_native_token \
		--other_chain_id 1 \
		--gas_amount 250000

gas-oracle-get-transaction-gas-cost-in-usd:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_transaction_gas_cost_in_usd \
		--other_chain_id 1 \
		--gas_amount 1

gas-oracle-crossrate:
	soroban contract invoke \
		--id $(GAS_ORACLE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		crossrate \
		--other_chain_id 1

#----------------POOL----------------------------

pool-deploy:
	soroban contract deploy \
          --wasm $(POOL_WASM_PATH_OP) \
          --source $(ADMIN_ALIAS) \
          --network $(NETWORK) 	\
          > $(POOL_ADDRESS_PATH) && echo $(POOL_ADDRESS)

pool-initialize:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN) \
        --bridge GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF \
        --a 20 \
        --token $(TOKEN_ADDRESS) \
        --fee_share_bp 10 \
        --balance_ratio_min_bp 0 \
        --admin_fee_share_bp 10

pool-set-bridge:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_bridge \
		--bridge $(BRIDGE_ADDRESS)

pool-deposit:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		deposit \
		--sender $(ADMIN) \
		--amount 10000000000

pool-get-pool-info:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_pool

pool-get-admin:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_admin

pool-get_pending_reward:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		pending_reward \
		--user $(ADMIN)

pool-get_user_deposit:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_user_deposit \
		--user $(ADMIN)

pool-get_claimable_balance:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_claimable_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

pool-claim_balance:
	soroban contract invoke \
		--id $(POOL_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		claim_balance \
		--user GB664P4XTBKNBK3YGPAFFCYPSW2SIO2FR6B6HC6SKFS7KGRTCDQYVUJ7

#---------------MESSENGER---------------------------
messenger-deploy:
	soroban contract deploy \
		--wasm $(MESSENGER_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(MESSENGER_ADDRESS_PATH) && echo $(MESSENGER_ADDRESS)

messenger-initialize:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN)\
        --chain_id 7 \
        --native_token_address $(NATIVE_ADDRESS)\
        --other_chain_ids 0001010101010100010101010101010101000000000000000000000000000000 \
        --gas_oracle_address $(GAS_ORACLE_ADDRESS)\
        --primary_validator_key 04734fc43dde79306ddf1bd5b4840d2cc9195bb48dbef92d081e71805694f5828d9cce76008f1f1a2a8a6ccd564b84937f83630d3d3af9541a5a3f3c1c1ea62c98 \
        --secondary_validator_keys '{ "04734fc43dde79306ddf1bd5b4840d2cc9195bb48dbef92d081e71805694f5828d9cce76008f1f1a2a8a6ccd564b84937f83630d3d3af9541a5a3f3c1c1ea62c98": true }'



messenger-set-gas-usage:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_gas_usage \
		--chain_id 10\
        --gas_usage 100000


send-message:
	soroban contract invoke \
		  --id $(MESSENGER_ADDRESS) \
		  --source alice \
		  --network $(NETWORK) \
		  -- \
		  send_message \
		  --message 0701efefefefefefefefefefefefefefefefefefefefefefefefefefefefefef \
		  --sender $(ALICE)

messenger-get-gas-usage:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_usage \
		--chain_id 1

messenger-get_transaction_cost:
	soroban contract invoke \
		--id $(MESSENGER_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_transaction_cost \
		--chain_id 1


#---------------BRIDGE---------------------------
bridge-deploy:
	soroban contract deploy \
		--wasm $(BRIDGE_WASM_PATH_OP) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		> $(BRIDGE_ADDRESS_PATH) && echo $(BRIDGE_ADDRESS)

bridge-initialize:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		initialize \
		--admin $(ADMIN) \
        --messenger $(MESSENGER_ADDRESS) \
        --gas_oracle $(GAS_ORACLE_ADDRESS) \
        --native_token $(NATIVE_ADDRESS) \

bridge-set-messenger:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_messenger \
		--messenger $(MESSENGER_ADDRESS)

bridge-set-gas-usage:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_gas_usage \
		--chain_id 10 \
		--gas_usage 250000

bridge-register-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		register_bridge \
#		--chain_id 10 \
#		--bridge_address 000000000000000000000000760d5d74bead2ccef05ccbfde32a08ebe7e4cfce
#		--chain_id 6 \
#		--bridge_address 000000000000000000000000c63c0261c2f1d21b3efe7828032e646c797ee21e
#		--chain_id 5 \
#		--bridge_address 000000000000000000000000763e75ca7bc589396f0e5c1b8049ac5ed7c8387f
#		--chain_id 4 \
#		--bridge_address 270a35d028b2940decaca3c3634f0bf4030c49a7a9a1c70c35bfa5dde5dd6208
#		--chain_id 3 \
#		--bridge_address 0000000000000000000000000e1de5c7267dc1c1bc498cc9bc3dbcaab305e8da
#		--chain_id 2 \
#		--bridge_address 000000000000000000000000aa8d065e35929942f10fa8cb58a9af24ee03655d
#		--chain_id 1 \
#		--bridge_address 000000000000000000000000a32196e86caa4e5d8bb44a7e7f162804421e38b7

bridge-add-bridge-token:
	soroban contract invoke \
    		--id $(BRIDGE_ADDRESS) \
    		--source $(ADMIN_ALIAS) \
    		--network $(NETWORK) 	\
    		-- \
    		add_bridge_token \
#    		--chain_id 10 \
#    		--token_address 000000000000000000000000ac7d9d0cc7da68f704a229a7258dc2ba654ffcbc
#    		--chain_id 10 \
#    		--token_address 00000000000000000000000097034742df00c506bd8b9f90e51330bf91ea59b4
#    		--chain_id 6 \
#    		--token_address 000000000000000000000000fd064A18f3BF249cf1f87FC203E90D8f650f2d63
#    		--chain_id 5 \
#    		--token_address 000000000000000000000000d18967827f4cc29193a7dbe2aa5ad440f6b27597
#    		--chain_id 5 \
#    		--token_address 0000000000000000000000003dbe838b635c54502c318f752187a8d8e7c73743
#    		--chain_id 4 \
#    		--token_address 3b442cb3912157f13a933d0134282d032b5ffecd01a2dbf1b7790608df002ea7
#    		--chain_id 4 \
#    		--token_address dc1f342783eef1ba0c9940714c5b5fe1a76d1f0f2ddab4a4faab53277e07dce3
#    		--chain_id 4 \
#    		--token_address 09c0917b1690e4929808fbc5378d9619a1ff49b3aaff441b2fa4bd58ab035a33
#    		--chain_id 3 \
#    		--token_address 0000000000000000000000003693bdbc20d9d8d0999b1d8effa686e88617e129
#    		--chain_id 3 \
#    		--token_address 0000000000000000000000003224f74a9e32e3f57c1b78a6aee79c257065110b
#    		--chain_id 2 \
#    		--token_address 0000000000000000000000000209da4a278956ca15438af8b108bd85642f096c
#    		--chain_id 2 \
#    		--token_address 00000000000000000000000049be77224dc061bd53699b25431b9aa7029a2cb8
#    		--chain_id 1 \
#    		--token_address 00000000000000000000000007865c6e87b9f70255377e024ace6630c1eaa37f
#    		--chain_id 1 \
#    		--token_address 000000000000000000000000c7dbc4a896b34b7a10dda2ef72052145a9122f43
#    		--chain_id 1 \
#    		--token_address 000000000000000000000000ddac3cb57dea3fbeff4997d78215535eb5787117

bridge-add-pool:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		add_pool \
		--pool $(POOL_ADDRESS) \
		--token $(TOKEN_ADDRESS)

bridge-set-rebalancer:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		set_rebalancer \
		--rebalancer GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF

bridge-swap-and-bridge:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		swap_and_bridge \
		--sender $(ADMIN) \
        --token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e \
        --amount 10000000 \
        --recipient 000000000000000000000000be959eed208225aab424505569d41bf3212142c0 \
        --destination_chain_id 1 \
        --receive_token 000000000000000000000000ddac3cb57dea3fbeff4997d78215535eb5787117 \
        --nonce 0000000000000000000000000000000000000000000000000000000000000011 \
        --gas_amount 10000000 \
        --fee_token_amount 0

bridge-swap:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		swap \
		--sender $(ADMIN) \
        --token cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e \
        --amount 10000000 \
        --recipient $(ADMIN) \
        --receive_token fea8431af9bb6bc27a45309a9db03f9ba478c4675a3d0579d18e303c3aaed561 \
        --receive_amount_min 0 \
        --claimable false

bridge-get_transaction_cost:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_transaction_cost \
		--chain_id 1

bridge-get_pool_address:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_pool_address \
		--token_address cb5cd675e2bb2f78d0b923fc555e8875b0b1ec1ecf0a03733133430d7b6b371e

bridge-get-config:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_config

bridge-get-gas-usage:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		get_gas_usage \
		--chain_id 1

bridge-has-received-message:
	soroban contract invoke \
		--id $(BRIDGE_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		has_received_message \
		--message 0107155a5bc1db9cb9d8fc56150518f01011f56ca2e3f0bdeb8dee115344d75b


#----------TOKEN--------------------------
token-transfer:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--source SBTECKZAIBLA6ZGPCG5IKON2IG4SJ37AVZEIY5OHCCKJ7KYCAJQKF5EB \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from GA2LLFIX5V3JT6IW67HH2JESPYALDGCV2AGCSEQOOEMKMF5K3WL2K7OS \
		--to $(ADMIN) \
		--amount 10000000000000

token-native-transfer:
	soroban contract invoke \
		--id $(NATIVE_ADDRESS) \
		--source $(ADMIN_ALIAS) \
		--network $(NETWORK) 	\
		-- \
		transfer \
		--from $(ADMIN) \
		--to $(BRIDGE_ADDRESS) \
		--amount 1000000000

token-get-balance:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		balance \
		--id $(POOL_ADDRESS)


token-get-name:
	soroban contract invoke \
		--id $(TOKEN_ADDRESS) \
		--network $(NETWORK) 	\
		-- \
		name

wrap-token:
	soroban lab token wrap \
		--network $(NETWORK) 	\
		--asset USDY:GAYODJWF27E5OQO2C6LA6Z6QXQ2EYUONMXFNL2MNMGRJP6RED2CPQKTW

native-token-address:
	soroban lab token id \
 		--network $(NETWORK) \
 		--asset native

generate-types:
	soroban contract bindings typescript \
	--network $(NETWORK) \
	--output-dir ./types/messenger \
	--wasm $(MESSENGER_WASM_PATH_OP) \
	--contract-id $(MESSENGER_ADDRESS)
