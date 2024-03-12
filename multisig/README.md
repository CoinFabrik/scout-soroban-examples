# Multisig Soroban Smart Contract

## Overview

This folder contains the Soroban smart contract example for Multisig. Multisig is a digital signature scheme that allows multiple individuals or entities to jointly authorize transactions or actions. It adds an extra layer of security and control to digital assets, such as cryptocurrencies or digital contracts.

## Contract Functions

###  `pub fn initialize_multisig(env: Env, owners: Vec<Address>, required_signatures: u32)`
Initializes the multisig wallet with a set of owners and a required number of signatures for transactions.

###  `pub fn approve_owner_addition(env: Env, owner: Address, caller:Address)`
Allows an existing owner to approve the addition of a new owner to the multisig wallet.

###  `pub fn approve_owner_removal(env: Env, owner: Address, caller:Address)`
 Allows an existing owner to approve the removal of another owner from the multisig wallet.
 
###  `fn add_owner(env: Env, new_owner: Address)`
Adds a new owner to the multisig wallet.

###  `fn remove_owner(env: Env, owner: Address)`
 Removes an existing owner from the multisig wallet.
 
###  `pub fn submit_tx(env: Env, token: Address, to: Address, amount: i128, caller: Address)`
Submits a transaction to be approved by the multisig owners. It records the transaction details and increments the transaction ID.

###  `pub fn confirm_transaction(env: Env, tx_id: TransactionId, owner: Address)`
Allows an owner to confirm a transaction by incrementing the confirmation count and recording the confirmation.

###  `pub fn execute_transaction(env: Env, tx_id: TransactionId)`
Executes a transaction once it has received the required number of confirmations.

###  `pub fn is_owner(env: Env, owner: Address) -> bool`
Checks if an address is an owner of the multisig wallet.

###  `pub fn get_multisig_state(env: Env) -> MultisigState`
Retrieves the current state of the multisig wallet.

## Interacting with the Contract

1. **Submit_tx** . The function submit_tx is called where the transaction is generated and the fields of token, caller, address, and amount are sent.
   
2. **Confirm_transaction** . The transaction is approved by the users through the function confirm_transaction. Each user can provide only one confirmation per transaction.

3. **Execute_transaction** . After the transaction has been generated and approved, the execute_transaction function carries out the transaction.
   
**Remove_owner / Add_owner** . If there is a need to delete or add an owner, both remove_owner and add_owner functions can be called. Signatures of the users are collected, and depending on the verdict, an owner is either removed or added

## Security Review
**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.
## About Soroban
Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).
soroban.stellar.orgsoroban.stellar.org
Welcome | Soroban - Smart Contracts Platform for Developers
Soroban is a smart contracts platform designed to be sensible, built-to-scale, batteries-included, and developer-friendly. (25 kB)
