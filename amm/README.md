# Automated Market Maker

## Overview

This folder contains a Soroban smart contract example for an automated market maker, which allows digital assets to be traded in a permissionless and automatic way by using liquidity pools rather than a traditional market of buyers and sellers. It demonstrates creating swaps, depositing, swapping and withdrawing tokens, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

_It is important to mention that inside this folder you will find the soroban example of the token used and also the two implemented curves which are constant product and constant sum._


## Contract Functions

| Function Name         | Parameters                                                                                       | Return Type              | Description                                                                  |
|-----------------------|--------------------------------------------------------------------------------------------------|--------------------------|------------------------------------------------------------------------------|
| `initialize`         | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>admin: Address</code></td></tr><tr><td><code>token_wasm_hash:  BytesN<32></td></tr></tbody></table> | `Result<(), SwapError>` | Initializes the contract with an admin and the token used. |
| `new_swap`            | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>admin: Address</code></td></tr><tr><td><code>token_a: Address</code></td></tr><tr><td><code>token_b: Address</code></td></tr><tr><td><code>swap_curve: Address</code></td></tr></tbody></table>                               | `Result<(), SwapError>`| Registers a token pair (token A and token B) for swapping and fixes the swap curve contract to be applied. |
| `deposit` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>from: Address</code></td></tr><tr><td><code>token_a: (Address, i128, i128)</code></td></tr><tr><td><code>token_b: (Address, i128, i128)</code></td></tr><tr><td><code>recipient: Address</code></td></tr></tbody></table>                                                                             | `Result<i128, SwapError>`| Deposits a pair of tokens such that the price of the pair does not change, and mints to `recipient` an amount of liquidity tokens contingent on the deposit. The tuples `token_a` and `token_b` contain three values: the token's address, the amount of tokens to be deposited, and the minimum amount required for the deposit to be acceptable by the caller.|
| `swap`             |  <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>from: Address</code></td></tr><tr><td><code>to: Address</code></td></tr><tr><td><code>token_a: Address</code></td></tr><tr><td><code>token_b: Address</code></td></tr><tr><td><code>input: i128</code></td></tr><tr><td><code>min_output: i128</code></td></tr></tbody></table>                                                                           | `Result<i128, SwapError>`| Swaps tokens using the configured swap curve to determine the amount of output tokens to be transferred to `to`, and performs the exchange if the minimum output requirement is met. |


## Interacting with the Contract

1. **Setting Up**. To configure the admin and token used, use the function `initialize()`.

2. **Create Swap**. To create a new swap with a token pair use `new_swap()`.

3. **Add liquidity**. In order to add liquidity, deposit tokens with the function `deposit()`.

4. **Swap Tokens**. For the user to swap their tokens use `swap()`.


_Take a look at the contract's test for a usage example._

## Security Review

:point_right: Navigate to [this link](https://github.com/CoinFabrik/scout-soroban-examples/blob/main/security-review/README.md) to view the security review.

