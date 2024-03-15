# Automated Market Maker

## Overview

This folder contains a Soroban smart contract example for an automated market maker, which allows digital assets to be traded in a permissionless and automatic way by using liquidity pools rather than a traditional market of buyers and sellers. It demonstrates creating swaps, depositing and swapping tokens, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

_It is important to mention that inside the folder you will find the soroban example of the token used and also the two implemented curves which are constant product and constant sum._


## Contract Functions

| Function Name         | Parameters                                                                                       | Return Type              | Description                                                                  |
|-----------------------|--------------------------------------------------------------------------------------------------|--------------------------|------------------------------------------------------------------------------|
| `initialize`         | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>admin: Address</code></td></tr><tr><td><code>token_wasm_hash:  BytesN<32></td></tr></tbody></table> | `Result<(), SwapError>` | Initializes the contract with an admin and the token used. |
| `new_swap`            | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>admin: Address</code></td></tr><tr><td><code>token_a: Address</code></td></tr><tr><td><code>token_b: Address</code></td></tr><tr><td><code>swap_curve: Address</code></td></tr></tbody></table>                               | `Result<(), SwapError>`| Creates a new swap pool for trading between the provided token pair (token A and token B) and defines the swap curve contract. |
| `deposit` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>from: Address</code></td></tr><tr><td><code>token_a: (Address, i128, i128)</code></td></tr><tr><td><code>token_b: (Address, i128, i128)</code></td></tr><tr><td><code>recipient: Address</code></td></tr></tbody></table>                                                                             | `Result<i128, SwapError>`| A pair of tuples is received with the address, quantity and minimum amount of tokens to be deposited. The function checks minimum deposit amounts and calculates liquidity units to be minted based on the geometric mean of deposited amounts. Liquidity tokens representing the user's share are minted and sent to the `recipient` |
| `swap`             |  <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>from: Address</code></td></tr><tr><td><code>to: Address</code></td></tr><tr><td><code>token_a: Address</code></td></tr><tr><td><code>token_b: Address</code></td></tr><tr><td><code>input: i128</code></td></tr><tr><td><code>min_output: i128</code></td></tr></tbody></table>                                                                           | `Result<i128, SwapError>`| Enables users to swap tokens within the defined pool. The contract uses the configured swap curve to determine the swap rate and performs the exchange if the minimum output requirement is met. |


## Interacting with the Contract

1. **Setting Up**. To configure the admin and token used, use the function `initialize()`.

2. **Create Swap**. To create a new swap with a token pair use `new_swap()`.

3. **Add liquidity**. In order to add liquidity, deposit tokens with the function `deposit()`.

4. **Swap Tokens**: For the user to swap his tokens use `swap()`.

## Security Review

> :warning: **This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.


