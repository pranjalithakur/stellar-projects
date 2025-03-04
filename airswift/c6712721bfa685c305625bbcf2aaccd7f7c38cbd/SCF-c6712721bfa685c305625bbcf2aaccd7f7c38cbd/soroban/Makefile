default: build

build:
	$(MAKE) -C ./scf_soroban || break;
	$(MAKE) -C ./token || break;
	$(MAKE) -C ./contract_deployer || break;
	$(MAKE) -C ./scf_pool || break;
	$(MAKE) -C ./argentina_pledge || break;
	$(MAKE) -C ./argentina_pool || break;