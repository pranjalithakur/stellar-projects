use soroban_sdk::{Address, Env, String, Vec};

use crate::storage_types::SplitRequest;

pub trait TokenizedCertificateTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Returns the current administrator
    fn admin(env: Env) -> Address;

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(env: Env, new_admin: Address);

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    /// Allows "operator" to manage token "id" if "owner" is the current owner of token "id".
    /// Emit event with topics = ["appr", operator: Address], data = [id: i128]
    fn appr(env: Env, owner: Address, operator: Address, id: i128);

    /// If "approved", allows "operator" to manage all tokens of "owner"
    /// Emit event with topics = ["appr_all", operator: Address], data = [owner: Address]
    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool);

    /// Returns the identifier approved for token "id".
    fn get_appr(env: Env, id: i128) -> Address;

    /// If "operator" is allowed to manage assets of "owner", return true.
    fn is_appr(env: Env, owner: Address, operator: Address) -> bool;

    /// Get the amount associated with "id".
    fn amount(env: Env, id: i128) -> u32;

    /// Get the parent id of "id" token.
    fn parent(env: Env, id: i128) -> i128;

    /// Get the owner of "id" token.
    fn owner(env: Env, id: i128) -> Address;

    /// Get the vc associated with "id".
    fn vc(env: Env, id: i128) -> Vec<String>;

    /// Get all TC ids owned by address
    fn get_all_owned(env: Env, address: Address) -> Vec<i128>;

    /// Get the "disabled" value of "id" token.
    fn is_disabled(env: Env, id: i128) -> bool;

    /// Transfer token "id" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(env: Env, from: Address, to: Address, id: i128);

    /// Transfer token "id" from "from" to "to", consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128);

    /// Mint the root-level TC. Will fail if the root-level TC already exists.
    /// The minted TC has a value corresponding to the "total_amount" specified in the initialize() function.
    /// Emit event with topics = ["mint", to: Address], data = [id: i128]
    fn mint_original(env: Env, to: Address, vc: String);

    /// Split a token into a number of sub-tokens based on the amounts listed. Will fail if the sum of amounts is greater than the original.
    /// Emit event with topics = ["split", from: Address], data = [id: i128, new_ids: Vec<i128>]
    fn split(env: Env, id: i128, splits: Vec<SplitRequest>) -> Vec<i128>;

    /// Burn a specified TC and transfer funds to the owner.
    /// Emit event with topics = ["burn", owner: Address], data = [id: i128]
    fn redeem(env: Env, id: i128);

    /// If "admin" is the administrator or the token owner, burn token "id" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [id: i128]
    fn burn(env: Env, id: i128);

    /// checks whether the payoff step was completed
    fn check_paid(env: Env) -> bool;

    /// use env timestamp and check against stored expiry time
    fn check_expired(env: Env) -> bool;

    /// set the contract address for the external token (e.g. USDC)
    fn set_external_token_provider(env: Env, contract_addr: Address, decimals: u32);

    /// retrieves a pending split request for a given token "id"
    fn recipient(env: Env, id: i128) -> Address;

    /// approve and receive the TC according to SplitRequest for "id"
    /// transfers the TC from the smart contract to the intended recipient
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn sign_off(env: Env, id: i128);

    /// pay off OrderInfo.amount using token
    fn pay_off(env: Env, from: Address);

    /// Update the VC associated with a token. Can only be called by the admin.
    fn add_vc(env: Env, id: i128, vc: String);

    // --------------------------------------------------------------------------------
    // Implementation Interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract.
    /// "admin" is the contract administrator.
    /// "buyer_address" specifies the account that will perform the pay-off step later.
    /// "total_amount" corresponds to the USD value of the invoice.
    /// "end_time" is a Unix timestamp. It specifies the maturity date of the invoice, after which the tokenized certificates can be redeemed for USDC or other tokens.
    fn initialize(e: Env, admin: Address, buyer_address: Address, total_amount: u32, end_time: u64);
}
