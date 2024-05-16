# Payment channel

## Overview

This folder contains a Soroban smart contract example for a payment channel. A payment channel is a smart contract that acts as a mediator between a person who wants to make a payment and another who has to receive that payment, in order to provide more security when carrying out the transaction.

## Contract Functions

| Function Name            | Parameters                                                                                                               | Return Type                | Description                                                                                              |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------|----------------------------|----------------------------------------------------------------------------------------------------------|
| `initialize`             | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>sender: Address</code></td></tr><tr><td><code>recipient: Address</code></td></tr><tr><td><code>token: Address</code></td></tr><tr><td><code>allowance: i128</code></td></tr><tr><td><code>expiration: Option<u32></code></td></tr></tbody></table> | `Result<PaymentChannelState, PCError>` | Initializes a new payment channel with provided parameters.                                             |
| `close`                  | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                        | `Result<(), PCError>`                   | Allows the recipient to close the channel and withdraw funds. Unclaimed funds will go back to the sender.                                           |
| `withdraw`               | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                        | `Result<(), PCError>`                   | Allows the recipient to withdraw funds incrementally from the channel.                                  |
| `set_expiration`         | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>sequence: u32</code></td></tr></tbody></table>                                                               | `Result<(), PCError>`                   | Sets the expiration timestamp of the channel.                                                            |
| `claim_timeout`          | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                                | `Result<(), PCError>`    | Allows the sender to claim funds if the channel has expired.                                            |
| `get_recipient_address`  | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                                | `Result<Address, PCError>`                | Retrieves the recipient's address associated with the payment channel.                                  |
| `get_sender_address`     | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                                | `Result<Address, PCError>`                | Retrieves the sender's address associated with the payment channel.                                     |
| `modify_allowance`       | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>amount: i128</code></td></tr></tbody></table> | `Result<(), PCError>`                   | Allows the sender to modify the permitted maximum amount to be withdrawn by the recipient (considering the sum of all partial extractions).                                    |
| `get_state`              | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                                | `Result<PaymentChannelState, PCError>`                | Retrieves the current state of the payment channel.                                  |


## Interacting with the Contract

1. **Initialization of Payment Channel**: Use the `initialize` function to initialize a new payment channel by providing the required parameters, such as sender address, recipient address, close duration, and token address.

2. **Making an Initial Payment**: Deposit funds into the payment channel by sending the desired amount to the contract.

3. **Partial Fund Withdrawal**: Make partial withdrawals of funds using the withdraw function.

4. **Closing the Payment Channel**: Close the channel and receive the remaining funds using the `close()` function and providing the desired amount.

 **Set Expiration**: The sender can set an expiration time for the channel using the `set_expiration()` function if desired.

 **Claim Timeout**: If the channel has expired and the funds have not been claimed, the sender can claim the remaining funds using the `claim_timeout()` function.

## Security Review

:point_right: Navigate to [this link](https://github.com/CoinFabrik/scout-soroban-examples/blob/main/security-review/README.md) to view the security review.


