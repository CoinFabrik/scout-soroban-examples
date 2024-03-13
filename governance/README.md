# Governance

## Overview

This folder contains the Soroban smart contract example for governance, where a member of a group can make a proposal and have the rest vote on it. It demonstrates starting, voting and closing a proposal, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

### `pub fn initialize(env: Env, supermajority: bool, supermajority_percentage: Option<u32>, voting_period: u64) -> GovernanceState` 

 This function sets up the initial state of the governance contract, including supermajority settings, voting period, and creates an empty GovernanceState object.

### `pub fn propose_tx(env: Env, contract_id: Address, func_name: Symbol, func_args: Vec<Val>)`

This function allows users to propose a transaction. It takes the target contract address, function name, arguments, and creates a proposal with an expiration date. 

### `pub fn vote_proposal(env: Env, voter: Address, proposal_id: u32, vote_value: bool)`

This function allows users to vote on a specific proposal with a yes or no vote. 
_It is important to mention that this contract does not put limitations on who can vote, this varies according to the group where the vote is executed._

### `pub fn close_proposal(env: Env, proposal_id: u32)`

This function allows any member to resolve the vote in case it is finished. It checks the voting results and executes the proposed transaction on the target contract if the vote passes based on the supermajority setting:
- **Supermajority**: If supermajority is enabled, a specified percentage of the total votes (defined by supermajority_percentage) must be in favor for approval.
- **Simple Majority**: If supermajority is disabled, a simple majority (more yes votes than no votes) is sufficient for approval.

### `pub fn get_state(env: Env) -> GovernanceState`

This function retrieves the current state of the governance system.

### Interacting with the Contract

1. **Setting Up**. To set up the governance contract with its corresponding configurations: `initialize`
2. **Create a Proposal**. Create the transaction to make a proposal: `propose_tx`
3. **Vote**. Vote yes or no on the generated proposal: `vote_proposal`
4. **Resolve Proposal**. If the proposal is already finished, resolve it: ` close_proposal`


## Security Review

![Security Audit Pending](https://example.com/security-audit-pending-banner.png)

**This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

## About Soroban

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).