# Multi Contract Caller

## Overview

This folder contains a Soroban smart contract example for a multi contract caller, consisting of one contract that calls two others that modify a fourth sotrage contract. It demonstrates interacting with another contract through an interface, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

### `pub fn init(env: Env, storage: Address, adder: Address, subber: Address) -> Result<(), CallerError>`

Initializes the state of the `CallerContract` by setting storage, adder, and subber addresses. It ensures that the contract is not already initialized before proceeding.

### `pub fn flip(env: Env) -> Result<(), CallerError>`

Flips the boolean value which is in the state, indicating a change in the operation mode.

### `pub fn variable_do_it(env: Env, x: i64) -> Result<i64, CallerError>`

Depending on the current state's which value, it selects either the `adder` or `subber` address and invokes the `do_it()` function through the `DoerClient`, passing the provided value `x`. This function returns the result of the invoked do_it operation.

**The following functions interact with ‘Adder’ and ‘Subber’ contracts, that its only function is to change a variable in ‘Storage’ contract**

### `pub fn do_it(env: Env, storage: Address, x: i64) -> i64`

This function is found in both the adder and subber contracts. It acts like a counter in both contexts. It reads a value from storage, and either subtracts (for subber) or adds (for adder) a given amount to it, stores the updated value back, and then returns the new value.

### `pub fn get(env: Env) -> i64`

### `pub fn set(env: Env, value: i64)`

These are the functions found in Storage contract, which only modify and access the value in memory.

## Interacting with the Contract

1. **Initialize The State**. Set up the `CallerContract` for the first time by assigning locations for its data and linking it to the adder and subber contracts: `init`

2. **Change The State**. Change the value of the `which` flag in the contract's state using the `flip()` function.

3. **Change Storage**. Retrieve the current state and then call the `do_it()` function on either the adder or subber contract (based on the flag) using the function `variable_do_it()`.

## Security Review

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).



