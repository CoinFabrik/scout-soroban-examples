# Vesting

## Overview

This folder contains a Soroban smart contract example for a vesting schedule, a digital agreement that governs the gradual transfer of ownership rights, typically for assets like company stock or crypto tokens. It demonstrates defining, adding, and withdrawing vesting schedules, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

| Function Name         | Parameters                                                                                       | Return Type              | Description                                                                  |
|-----------------------|--------------------------------------------------------------------------------------------------|--------------------------|------------------------------------------------------------------------------|
| `new_vesting`         | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>token: Address</code></td></tr><tr><td><code>beneficiary: Address</code></td></tr><tr><td><code>start_time: u64</code></td></tr><tr><td><code>duration: u64</code></td></tr><tr><td><code>admin: Address</code></td></tr></tbody></table> | `Result<u64, VestError>` | Initializes a new vesting schedule with specific parameters. start_time is a UNIX timestamp, while duration is in seconds. admin is the user who will be allowed to call add_vest(). |
| `add_vest`            | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>id: u64</code></td></tr><tr><td><code>token: Address</code></td></tr><tr><td><code>from: Address</code></td></tr><tr><td><code>amount: i128</code></td></tr></tbody></table>                               | `Result<i128, VestError>`| Adds funds to a specified vesting account. Pass as id the value returned by new_vesting().|
| `retrievable_balance` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>id: u64</code></td></tr></tbody></table>                                                                             | `Result<i128, VestError>`| Retrieves the balance that is currently withdrawable by the beneficiary from the vesting account. Pass as id the value returned by new_vesting().|
| `pay_out`             | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>id: u64</code></td></tr></tbody></table>                                                                             | `Result<i128, VestError>`| Transfers any currently available funds to the beneficiary's account. Note that anyone can call this function on behalf of the beneficiary. Pass as id the value returned by new_vesting().|


## Interacting with the Contract

1. **Create a Vesting Schedule**. To create and define the vesting schedule with its corresponding parameters use the function `new_vesting()`.

2. **Add Funds**. To add the necessary funds to the vesting use `add_vest()`.

3. **Check Balance**. In order to check the balance at the moment for a particular user, call `retrievable_balance()`.

4. **Withdraw Balance**: For the user to withdraw balance use `pay_out()`.

## Security Review

:point_right: Navigate to [this link](https://github.com/CoinFabrik/scout-soroban-examples/blob/main/security-review/README.md) to view the security review.

