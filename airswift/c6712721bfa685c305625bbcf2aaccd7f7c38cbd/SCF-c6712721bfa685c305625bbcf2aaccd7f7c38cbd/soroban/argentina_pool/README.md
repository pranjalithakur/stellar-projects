# Argentina Case Liquidity Pool

## Notes
* This smart contract involves three kinds of "token". 
    * The "liquidity token" is used to represent funds locked into the smart contract. They are used for creating and paying off loans.
    * The "external token" is only used to enter/exit the liquidity pool. Typically, this would be a stablecoin like USDC, which could be used to easily make transactions outside of the SCF platform. 
    * The "tokenized certificate" (TC) is a token from the `argentina_pledge` smart contract which represents an invoice on the SCF platform. It is non-fungible, and used as collateral for loans in this liquidity pool.
* A "loan" is expressed relative to the liquidity token. For example, the "creditor" is the one who gives liquidity tokens and receives a TC, while the "borrower" is the one who receives liquidity tokens in exchange for the TC. 

## Steps
1. Initialize the smart contract using `initialize`. The "token_wasm_hash" parameter comes from installing an instance of a generic token smart contract (The `../token/` contract is the example token contract from the official Stellar examples repository). The "rate_percent" parameter, if set to a value above 0, will increase the amount needed to pay off the loan. 
2. The creditor calls `deposit` to deposit some USDC to the smart contract and receive an equal number of liquidity tokens. 
3. The creditor calls `create_loan_offer` to offer to loan liquidity tokens to a TC holder in exchange for their TC as collateral. To create a loan offer, the creditor must transfer liquidity tokens to the smart contract equal to the "amount" value associated with that TC.
    * The creditor can retrieve their liquidity tokens from the smart contract by cancelling the loan offer. `cancel_loan_offer` can be called by the same creditor as long as the offer hasn't been accepted yet.
    * The pool's payoff rate can be changed by the admin via `set_rate`. The loan's rate will always be the rate that the pool had when the offer was created, even if the pool rate changes afterwards.
4. The owner of the TC (borrower) can accept using `accept_loan_offer`. This transfers the liquidity tokens to the borrower and transfers the TC to the creditor.
5. The borrower is able to use `withdraw` to trade their loaned liquidity tokens for USDC. 
6. The borrower must use `payoff_loan` to send liquidity tokens to the smart contract before they are able to get their TC back. If the loan rate is greater than 0, the borrower must pay back more liquidity tokens than they originally received from the creditor. 
7. The creditor calls `close_loan` to return the TC to the borrower and receive the number of liquidity tokens paid during `payoff_loan`. 
8. The creditor is able to `withdraw` the liquidity tokens for USDC.
