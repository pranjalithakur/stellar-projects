# Supply Chain Finance Tokenized Certificate

## Steps
1. Initialize the contract using `initialize`.
2. Set the external token used for payoff/redeem using `set_external_token_provider`. On a local or test network, it may be advisable to use a token contract or mint your own asset and wrap it.
3. Using the admin account, mint the root-level tokenized certificate of the original invoice using `mint_original`. This function can only be called once. The "VC" parameter is intended to store a Verifiable Credential as a JSON string, to be generated and retrieved by systems interacting through the smart contract. For testing purposes, it can be an arbitrary string. 
4. As long as the current ledger time is earlier than `end_time`, the owner of a given tokenized certificate can use `split` to generate new tokenized certificates with portions of the original tokenized certificate's value. See the below "SplitRequest Format" section for details on how to call this function.
5. The intended recipient of a "split" tokenized certificate can accept the split using the `sign_off` function. Upon calling this function, ownership of the tokenized certificate is transferred to the recipient. Otherwise, the tokenized certificate remains owned by the contract's address until `end_time`.
6. After the ledger time passes `end_time`, any call to `check_expired` or ownership/transfer-related functions will auto-transfer unclaimed split tokenized certificates to the owner of its parent certificate.
7. The `buyer_address` specified during `initialize` can call the `pay_off` function to transfer tokens to the contract address equal to the `total_amount` (also specified during initialization).
8. If the maturity date has been reached and payoff has been completed, the owner of a tokenized certificate can call `redeem` to burn it in exchange for an equivalent balance in external tokens.

### "SplitRequest" Format
If you are using the CLI, you can specify the SplitRequest arguments as an array of maps. In this example, the command would split token 0 and generate 3 new tokens. Token 1 will have value 200000, while token 2 will have value 300000. Assuming Token 0 had an initial balance of 1000000, token 3 will be generated to hold the remaining amount of 500000, and it will have the same owner as token 0. Token 0 will then be marked as disabled.
```
soroban contract invoke --network scf-demo --id CDBER4ZI4CDADMBAICCTVYOI3M7UVPQJK4DEXWEF44BDSRX76QJMAPHV --source-account admin -- split --id 0 --splits '[{"amount":200000,"to":"GB4PXUF5BB5SI2P7PVUYOIGFMCVCVWITKDAJUQAZTQIIAGIMAPPRA45H"}, {"amount":300000,"to":"GCY4TVGX72CXLWDVRPTKK7H5LY32NYIP4G7QHSRY2NPFOL7W5VJGHF5B"}]'
```
