# Multisig

## Overview

This folder contains a Soroban smart contract example for a multisig. A multisig is a digital signature scheme that allows multiple individuals or entities to jointly authorize transactions. It adds an extra layer of security and control to digital assets, such as cryptocurrencies or digital contracts.

## Contract Functions

| Function Name            | Parameters                                                                                                             | Return Type         | Description                                                                              |
|--------------------------|------------------------------------------------------------------------------------------------------------------------|---------------------|------------------------------------------------------------------------------------------|
| `initialize_multisig`    | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>owners: Vec&lt;Address&gt; </code></td></tr><tr><td><code>required_signatures: u32</code></td></tr></tbody></table> | `None`              | Initializes the multisig wallet with a set of owners and a required number of signatures for transactions. |
| `approve_owner_addition` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>owner: Address</code></td></tr><tr><td><code>caller: Address</code></td></tr></tbody></table>                | `None`              | Allows an existing owner to approve the addition of a new owner to the multisig wallet.   |
| `approve_owner_removal`  | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>owner: Address</code></td></tr><tr><td><code>caller: Address</code></td></tr></tbody></table>                | `None`              | Allows an existing owner to approve the removal of another owner from the multisig wallet.|
| `add_owner`              | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>new_owner: Address</code></td></tr></tbody></table>                                                          | `None`              | Adds a new owner to the multisig wallet.                                                  |
| `remove_owner`           | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>owner: Address</code></td></tr></tbody></table>                                                             | `None`              | Removes an existing owner from the multisig wallet.                                       |
| `submit_tx`              | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>token: Address</code></td></tr><tr><td><code>to: Address</code></td></tr><tr><td><code>amount: i128</code></td></tr><tr><td><code>caller: Address</code></td></tr></tbody></table> | `None` | Submits a transaction to be approved by the multisig owners.                             |
| `confirm_transaction`    | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>tx_id: TransactionId</code></td></tr><tr><td><code>owner: Address</code></td></tr></tbody></table>            | `None`              | Allows an owner to confirm a transaction.                                                 |
| `execute_transaction`    | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>tx_id: TransactionId</code></td></tr></tbody></table>                                                       | `None`              | Executes a transaction once it has received the required number of confirmations.        |
| `is_owner`               | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>owner: Address</code></td></tr></tbody></table>                                                             | `bool`              | Checks if an address is an owner of the multisig wallet.                                  |
| `get_multisig_state`     | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                              | `MultisigState`     | Retrieves the current state of the multisig wallet.                                       |


## Interacting with the Contract

1. **Submit Transaction**. Submit a transaction using the function `submit_tx()`.
   
2. **Confirm Transaction**. Approve the transaction through the function `confirm_transaction()`. Each user can provide only one confirmation per transaction.

3. **Execute Transaction** . After the transaction has been generated and approved, use the `execute_transaction()` function to carry out the transaction.
   
**Remove Owner or Add Owner**. Whenever there is the need to add or remove an owner, current members should vote on it by calling `approve_owner_adittion` or `approve_owner_removal`. Every time a member invokes one of these functions, a verification will be done to check whether the removal or the addition has reached the minimum amount of signatures required to be executed. If the required confirmations have been, indeed, reached these functions will take care of calling the internal methods `add_owner` or `remove_owner` respectively. 

## Security Review

> :warning: **This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

