# Vesting Soroban Smart Contract

## Overview

This folder contains the Soroban smart contract example for vesting, a digital agreement that governs the gradual transfer of ownership rights, typically for assets like company stock or crypto tokens.It demonstrates defining, adding, and withdrawing vesting schedules, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

### `new_vesting`: creates a new vesting schedule with specified parameters

### `add_vest`: add funds to vesting

### `retrievable_balance`: retrieves the balance that a user can currently withdraw from their vesting schedule

### `pay_out`: transfers the vested funds to the beneficiary


### Interacting with the Contract

**Create a Vesting Schedule**. Create and define the vesting with its corresponding parameters: `new_vesting`
**Add Funds**. Add the necessary funds to the vesting: `add_vest`
**Check Balance**. In order to check the balance at the moment, the user can call:  `retrievable_balance`
**Withdraw Balance**: Allows the user to withdraw their balance: `pay_out`

## Security Review

![Security Audit Pending](https://example.com/security-audit-pending-banner.png)

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).
