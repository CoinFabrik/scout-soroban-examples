# Vesting

## Overview

This folder contains a Soroban smart contract example for vesting, a digital agreement that governs the gradual transfer of ownership rights, typically for assets like company stock or crypto tokens. It demonstrates defining, adding, and withdrawing vesting schedules, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

### `pub fn new_vesting(env: Env, token: Address, beneficiary: Address, start_time: u64, duration: u64, admin: Address) -> Result<u64, VestError>` 

Creates a new vesting schedule with specified parameters.

### `pub fn add_vest(env: Env, id: u64, token: Address, from: Address, amount: i128) -> Result<i128, VestError>`

Add funds to vesting.

### `pub fn retrievable_balance(env: Env, id: u64) -> Result<i128, VestError>`

Retrieves the balance that a user can currently withdraw from their vesting schedule.

### `pub fn pay_out(env: Env, id: u64) -> Result<i128, VestError>`

Transfers the vested funds to the beneficiary.


## Interacting with the Contract

1. **Create a Vesting Schedule**. To create and define the vesting with its corresponding parameters use the function `new_vesting()`.

2. **Add Funds**. To add the necessary funds to the vesting use `add_vest()`

3. **Check Balance**. In order to check the balance at the moment, call `retrievable_balance()`.

4. **Withdraw Balance**: For the user to withdraw the balance use `pay_out()`.

## Security Review

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).
