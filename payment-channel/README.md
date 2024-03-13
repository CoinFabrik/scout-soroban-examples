# Payment channel

## Overview

This folder contains a Soroban smart contract example for a payment channel. A payment channel is a smart contract that acts as a mediator between a person who wants to make a payment and another who has to receive that payment, in order to provide more security when carrying out the transaction.

## Contract Functions

### `pub fn initialize(env: Env, sender: Address, recipient: Address, close_duration: i128, token: Address) -> PaymentChannelState`

Initializes a new payment channel with provided parameters and stores its state in the contract's storage.

### `pub fn close(env: Env, amount: i128, signature: BytesN<64>)`

Allows the recipient to close the channel and withdraw funds.

### `pub fn withdraw(env: Env, amount: i128, signature: BytesN<64>)`

Allows the recipient to withdraw funds incrementally from the channel.

### ` pub fn set_expiration(env: Env, timestamp: i128)`

Allows the recipient to close the channel and withdraw funds.

### ` pub fn claim_timeout(env: Env) -> Result<(), PCError>`

Allows the sender to claim funds if the channel has expired.

### `pub fn get_recipient_address(env: Env) -> Address`

Retrieves the recipient's address associated with the payment channel.

### `pub fn get_sender_address(env: Env) -> Address`

Retrieves the sender's address associated with the payment channel.

### `fn verify_signature(env: Env, amount: i128, signature: BytesN<64>, sender_pubkey: Address)`

Validates the signature of a transaction to ensure its authenticity.

## Interacting with the Contract

1. **Initialization of Payment Channel** : Use the initialize function yo initialize a new payment channel by providing the required parameters, such as sender address, recipient address, close duration, and token address.

2. **Making an Initial Payment**: Deposit funds into the payment channel by sending the desired amount to the contract.

3. **Partial Fund Withdrawal**: Make partial withdrawals of funds using the withdraw function, providing the desired amount and corresponding digital signature.

4. **Closing the Payment Channel**: Close the channel and receive the remaining funds using the `close()` function and providing the desired amount and corresponding digital signature.

 **set_expiration**: The sender can set an expiration time for the channel using the `set_expiration()` function if desired.

 **claim_timeout**: If the channel has expired and the funds have not been claimed, the sender can claim the remaining funds using the `claim_timeout()` function.

## Security Review

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).





