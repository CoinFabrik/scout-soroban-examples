# Multi Contract Caller

## Overview

This folder contains a Soroban smart contract example for a multi contract caller, consisting of one contract that calls two others that modifying a storage contract. It demonstrates interacting with another contract through an interface, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

| Contract     | Function Name    | Parameters                                                                                                     | Return Type               | Description                                                                                                                |
|--------------|------------------|----------------------------------------------------------------------------------------------------------------|---------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `caller`       | `init`           | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>storage: Address</code></td></tr><tr><td><code>adder: Address</code></td></tr><tr><td><code>subber: Address</code></td></tr></tbody></table> | `Result<(), CallerError>` | Initializes the state of the `caller` contract with addresses for the `storage`, `adder`, and `subber` contracts.                              |
| `caller`       | `flip`           | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                         | `Result<(), CallerError>` | Flips the boolean value in the state, indicating a change in operation mode.                                               |
| `caller`       | `variable_do_it` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>x: i64</code></td></tr></tbody></table>       | `Result<i64, CallerError>`| Selects either the `adder` or `subber` contract based on the current state and invokes the `do_it()` function with the value `x`.       |
| `adder`        | `do_it`          | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>storage: Address</code></td></tr><tr><td><code>x: i64</code></td></tr></tbody></table>                      | `i64`                     | Adds a given amount to a value from storage, updates it, and returns the new value.                                        |
| `subber`       | `do_it`          | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>storage: Address</code></td></tr><tr><td><code>x: i64</code></td></tr></tbody></table>                      | `i64`                     | Subtracts a given amount from a value in storage, updates it, and returns the new value.                                   |
| `storage`      | `get`            | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                         | `i64`                     | Accesses and returns a value from the `storage` contract.                                                                    |
| `storage`      | `set`            | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>value: i64</code></td></tr></tbody></table>  | `None`                    | Modifies a value in the `storage` contract.                                                                                  |


## Interacting with the Contracts

Deploy the four smart contracts.

1. **Initialize State**. Set up the `caller` for the first time with the `init()` function, by assigning locations for its data (`storage` contract) and linking it to the `adder` and `subber` contracts.

2. **Change State**. Change the value of the `which` flag in the contract's state using the `flip()` function.

3. **Change Storage**. Use the function `variable_do_it()` to retrieve the current state and call the `do_it()` function on either the adder or subber contract (based on the flag).

## Running tests

To run the unit tests first go into each contract's directory and build the contract with `soroban contract build`. Then run the tests with `cargo test` in each directory. `full_test.sh` is provided for convenience, which executes the build and test steps in a correct order.

## Security Review

:point_right: Navigate to [this link](https://github.com/CoinFabrik/scout-soroban-examples/blob/main/security-review/README.md) to view the security review.


