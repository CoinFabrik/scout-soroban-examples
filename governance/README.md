# Governance

## Overview

This folder contains a Soroban smart contract example for governance, where a member of a group can make a proposal and have the rest vote on it. It demonstrates starting, voting and closing a proposal, showcasing how to implement and interact with smart contracts on the Stellar network using Soroban.

## Contract Functions

| Function Name    | Parameters                                                                                                                       | Return Type          | Description                                                                                                                  |
|------------------|----------------------------------------------------------------------------------------------------------------------------------|----------------------|------------------------------------------------------------------------------------------------------------------------------|
| `initialize`     | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>supermajority: bool</code></td></tr><tr><td><code>supermajority_percentage: Option&lt;u32&gt;</code></td></tr><tr><td><code>voting_period: u64</code></td></tr></tbody></table> | `Result<(), GovError>` | Sets up the initial state of the governance contract, including supermajority settings and voting period.                    |
| `propose_tx`     | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>contract_id: Address</code></td></tr><tr><td><code>func_name: Symbol</code></td></tr><tr><td><code>func_args: Vec&lt;Val&gt;</code></td></tr></tbody></table>                      | `None`             | Allows users to propose a transaction by specifying the target contract, function name, and arguments.                       |
| `vote_proposal`  | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>voter: Address</code></td></tr><tr><td><code>proposal_id: u32</code></td></tr><tr><td><code>vote_value: bool</code></td></tr></tbody></table>                              | `None`             | Enables users to vote on proposals with a `yes` or `no` vote.                                                               |
| `close_proposal` | <table><tbody><tr><td><code>env: Env</code></td></tr><tr><td><code>proposal_id: u32</code></td></tr></tbody></table>                                                                    | `None`             | Resolves a vote, checking results and executing the transaction on the target contract if the vote passes the majority rule. |
| `get_state`      | <table><tbody><tr><td><code>env: Env</code></td></tr></tbody></table>                                                                                        | `GovernanceState` | Retrieves the current state of the governance system.                                                                        |


## Interacting with the Contract

1. **Setting Up**. Use the `initialize` function to set up the governance contract with its corresponding configurations.
2. **Create a Proposal**. Create the transaction to make a proposal using the function `propose_tx()`.
3. **Vote**. Vote yes or no on the generated proposal using the function `vote_proposal()`.
4. **Resolve Proposal**. If the proposal voting period has already finished, resolve it using the function ` close_proposal()`.

## Other considerations 

- Current Governance example allows proposals with **one transaction only**.
- Current Governance example does not verify members belong to a specific community and must be customized to do it (either by owning a *governance token* or other possible criteria).
- Governance is intended to have authority over other contracts inside a system.

## Security Review

> :warning: **This Smart Contract is pending to be audited in April 2024.** Use at your own risk. Contributions and bug reports are welcome to enhance the security and functionality of this contract.

