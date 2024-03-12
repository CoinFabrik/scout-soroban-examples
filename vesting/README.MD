# Vesting Soroban Smart Contract

## Overview

This folder contains the Soroban smart contract example for vesting, a digital agreement that governs the gradual transfer of ownership rights, typically for assets like company stock or crypto tokens.It demonstrates defining, adding, and withdrawing vesting schedules, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

### `pub fn new_vesting(env: Env, token: Address, beneficiary: Address, start_time: u64, duration: u64, admin: Address) -> Result<u64, VestError>` 

creates a new vesting schedule with specified parameters

### `pub fn add_vest(env: Env, id: u64, token: Address, from: Address, amount: i128) -> Result<i128, VestError>`

add funds to vesting

### `pub fn retrievable_balance(env: Env, id: u64) -> Result<i128, VestError>`

retrieves the balance that a user can currently withdraw from their vesting schedule

### `pub fn pay_out(env: Env, id: u64) -> Result<i128, VestError>`

transfers the vested funds to the beneficiary


## Interacting with the Contract

1. **Create a Vesting Schedule**. Create and define the vesting with its corresponding parameters: `new_vesting`

2. **Add Funds**. Add the necessary funds to the vesting: `add_vest`

3. **Check Balance**. In order to check the balance at the moment, the user can call:  `retrievable_balance`

4. **Withdraw Balance**: Allows the user to withdraw their balance: `pay_out`

## Security Review

![Security Audit Pending](https://example.com/security-audit-pending-banner.png)

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).
