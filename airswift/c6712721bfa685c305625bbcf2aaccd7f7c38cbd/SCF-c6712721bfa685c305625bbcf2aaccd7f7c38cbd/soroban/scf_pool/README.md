# TC Offer Pool Smart Contract

This smart contract is for making offers on the tokenized certificates (TC) of the `scf_soroban` smart contract, as well as locking in funds by exchanging stablecoins for liquidity tokens.

## Steps
1. `initialize`: Set the admin, and also specify the address for a token that can be used to exchange for a TC during loaning (e.g. USDC contract address).
2. `add_pool_token`: Add support for an external stablecoin token (such as USDC) by creating a liquidity token to exchange for it. Each supported external token has its own liquidity token, which cannot be used interchangeably with other liquidity tokens.
3. `deposit`: A user can deposit an `amount` of a supported `ext_token` to the smart contract to mint and receive an equal number of the associated liquidity token.
4. `create_offer`: Offer an `amount` of liquidity tokens for a specific TC. These liquidity tokens will be held by the smart contract until the offer is accepted or cancelled. The system calling the smart contract is responsible for generating the `offer_id` and storing it.
5. `accept_offer`: Must be called by the owner of the TC targeted by the offer. Transfers the TC to the offerer, and transfers the offered liquidity tokens from the smart contract to the TC's original owner.
6. `withdraw`: A user can burn an `amount` of liquidity tokens to receive an equal number of external tokens.

#### Other functions
* `expire_offer`: Can be called by the admin or the creator of a given offer. Cancels the offer and returns the offered tokens to the offerer.
* `get_offer`
* `get_pool_tokens`: Return a map of stablecoin address -> liquidity token address.
* `get_ext_token`: Return the address of the stablecoin associated with a given liquidity token.