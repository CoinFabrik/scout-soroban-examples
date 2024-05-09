# Security Review

## Executive Summary

In the context of the grant awarded by the [Stellar Community Fund](https://communityfund.stellar.org/), we tasked developers with no prior experience in Soroban to write smart contracts within a time-constrained environment, deliberately inducing mistakes.

These smart contracts were then subjected to review by one of CoinFabrik's senior security auditors to identify issues and suggest enhancements.

This initiative served a dual purpose: first, it allowed us to simulate real-world development conditions and test Scout against authentic scenarios. Second, the development of these smart contracts enabled us to explore more intricate vulnerabilities that were not covered in the Proof of Concept (PoC) and Prototype phases, inspiring the creation of new vulnerability classes, test cases, and detectors to be incorporated into Scout.

Additionally, we aim for this report to benefit the Soroban developer community by highlighting typical issues that arise during the development of a series of smart contracts. These examples should assist developers in writing more secure and robust code.

Git commit reviewed: `e3ffdb9f75d56992023b8b747bab81878f137746`

Tests were not included in the scope of this review.

:warning: This is not a full security audit.

## Reported issues

### IS-01 Bad Cargo.lock Tracking

- Location:
  - `multisig/Cargo.lock`
  - `amm/.gitignore:2`
  - `vesting/.gitignore:2`
  - Non-committed `Cargo.lock` files.

Only 3 `Cargo.lock` files history is managed by git (`governance/governance/Cargo.lock`, `governance/mock-contract/Cargo.lock` and `multisig/Cargo.lock`). One of those is altered when running cargo test in the directory (`multisig/Cargo.lock`). The rest is generated when using `cargo` normally. Also, some `Cargo.lock` files are intentionally ignored (see `amm/.gitignore:2` and `vesting/.gitignore:2`).

Not properly managing the `Cargo.lock` files may lead to unintentionally including vulnerable code in the dependencies.

####  Recommendation

1. Commit all `Cargo.lock` files.
2. Remove all `Cargo.lock` references from `.gitignore` files.
3. Make sure that running `cargo build` and/or `cargo test` does not alter any `Cargo.lock` files.
4. Add this check to the CI.

#### Status

:heavy_check_mark: Resolved

### IS-02 Failing Tests May Be Ignored

- Location:
  - `multi-contract-caller/full_test.sh`

When running the mentioned file a test failure in the adder or subber directories may be hidden because the script will keep running. Also, if the script is used in a CI process it will not detect a failing test, as it ignores return codes.

#### Recommendation

Add `set -e` after the hash-bang line to fail when a command generates a non-zero return code.

#### Status

:x: Rejected: (`full_test.sh` is not intended as a test suite, but more as runnable documentation.)

### IS-03 vesting Overflow

- Location:
  - `vesting/src/lib.rs:102-103`

The multiplication in lines 102 and 103, inside the `retrievable_balance_internal` function,

```rust
state.locked.checked_mul(now.into())
```

may lead to an overflow if the vesting is long enough and sufficient time has passed, when calculating the vesting balance. This, in turn, may trigger the loss of funds vested that were not retrieved when the overflow occurs as the pay_out function uses the retrievable_balance_internal function.

#### Recommendation

Either restrict the duration and the locked amount so that its multiplication does not overflow an i128 or make the operation using 256-bits integers. The final value returned by the retrievable_balance_internal function can be a 128-bits integer, as now is always lower (or equal) than duration, ensuring that the calculated value fits. An implementation of the multiplication and division operation using 256 bits is made in the `safe_mul` function defined in `amm/util/src/rational.rs:95`.

#### Status

:heavy_check_mark: Resolved

### IS-04 payment-channel Lack of Funds

- Location:
  - `payment-channel/src/lib.rs: 37-49, 108-114`
 
When the payment channel contract is initialized via the initialize function, or when the allowance is increased via the modify_allowance function there is nothing in the code that ensures that the funds to be sent to the recipient are available. This may lead to the sender not honoring the promise of payment.

#### Recommendation

Either make sure in the contract that the funds are available or make the transfer funding the channel on the two functions.

#### Status

:heavy_check_mark: Resolved

### IS-05 Sender May Deny Funds After Commitment in payment-channel

- Location:
  - `payment-channel/src/lib.rs: 81-88, 90-105`

A sender may deny the receiver of the committed funds (as defined in the `allowance` field) by setting the expiration as `env.ledger().timestamp() + 1` via the `set_expiration` function and then getting all the contract balance by invoking the `claim_timeout` function.

#### Recommendation

The `set_expiration` function should only allow setting a new expiration after the old one. 

If the expiration is not set it should be handled as if the channel cannot expire.

The `initialize` function should receive an expiration that can be either a timestamp or `None`.

#### Status

:heavy_check_mark: Resolved.

### IS-06 multisig Transaction Leak

- Location:
  - `multisig/src/lib.rs`

All the transactions state is stored in the instance state, which is capped at 64Kb. This issue is made worse by the fact that the data of all transactions already executed are kept in the storage stored in the instance store.

This issue allows a single owner to completely and permanently halt the operation of the contract forever by submitting enough transactions.

#### Recommendation

Keep all the data for each transaction on its own persistent entry. This includes, at least, the data in the `confirmations`, `confirmation_count` and `transactions` maps.

#### Status

:heavy_check_mark: Resolved.

### IS-07 multisig Proposed Owners Leak

- Location:
  - `multisig/src/lib.rs`

All the proposals to change the owners (by either adding or removing them) are stored in the instance state, which is capped at 64Kb.
This issue allows a single owner to completely and permanently halt the operation of the contract forever by submitting enough owner proposals.

#### Recommendation

Keep all the data for each owner-change proposal on its own persistent entry. This includes, at least the data in the `pending_modifications` and `owner_modifications_conf` maps.

#### Status

:heavy_check_mark: Resolved

### IS-08 Zero Owners in multisig

- Location:
  - `multisig/src/lib.rs: 37-50`

The multisig contract may get 0 owners by initializing it with 0 owners via the `initialize_multisig`.

A multisig contract with 0 owners cannot operate.

#### Recommendation

Check on initialization that the contract has at least one owner.

#### Status

:heavy_check_mark: Resolved.

### IS-09 Not Enough Owners in multisig

- Location:
  - multisig/src/lib.rs: 37-50, 107, 113-126

The multisig contract may get less owners than required to operate in the `required_signatures` setting  by either initializing it with fewer owners than required via the `initialize_multisig` function or removing enough owners via the `approve_owner_removal` function.

A multisig contract with less owners than required cannot operate.

#### Recommendation

Check on initialization that there are enough owners. On owner removal if there are not enough owners reduce the number of required owners to do an action.

#### Status

:heavy_check_mark: Resolved.

### IS-10 No Auth in multi-contract-caller

- Location:
  - multi-contract-caller/**/*.rs

No authorization has been implemented in any of the contracts in the `multi-contract-caller` directory.

In particular, any value can be written by any actor in any instance of the storage contract.

#### Recommendation

Properly authorize access in the contracts.

#### Status

:x: Rejected: `multi-contract-caller` is purposely a simple contract, meant to demonstrate a variable interface.

### IS-11 Avoid Timestamps

- Location:
  - `governance/governance/src/lib.rs: 74, 91, 109`
  - `payment-channel/src/lib.rs:83, 95`
  - `vesting/src/lib.rs:82`

The builder of the current block may slightly alter the timestamp, leaving the code that uses it vulnerable to expiring something or not depending on the timestamp given by the builder.

See [https://stellar.stackexchange.com/questions/632/how-are-timestamps-deemed-invalid](https://stellar.stackexchange.com/questions/632/how-are-timestamps-deemed-invalid) for details on how the timestamp can be manipulated.

#### Recommendation

Use `env.ledger().sequence()` instead of `env.ledger.timestamp()` in logic where actions expire.

#### Status

:heavy_check_mark: **Resolved** for governance and payment-channel contracts.

:x: Rejected for vesting contract: Replacing timestamp for block height would make it inconvenient to set up future vesting periods.

### IS-12 No Auth in governance.vote_proposal

- Location:
  - `governance/governance/src/lib.rs: 88-104`

Any account can cast a vote on behalf of any other account because the `vote_proposal` function has no authentication or authorization mechanism implemented.

#### Recommendation

Add the `voter.require_auth();` statement to the function.

#### Status

:heavy_check_mark: Resolved.

### IS-13 No Voter Whitelist Mechanism in governance

- Location:
  - `governance/governance/src/lib.rs`

There is no mechanism to restrict the accounts that can cast votes in the governance, allowing an attacker to propose and approve any proposal by generating enough accounts.

#### Recommendation

Whitelist the accounts that can cast votes in the governance mechanism.

#### Status

:heavy_check_mark: Resolved.

IS-14 No governance Quorum

- Location:
  - `governance/governance/src/lib.rs
`
There is no minimum number of votes required to approve a proposal, nor a minimum time period time given to vote a proposal before approval. This allows an attacker to do any proposal by invoking `propose_tx`, `vote_proposal` and `close_proposal` in quick succession, maybe even in the same transaction.

#### Recommendation

Implement a quorum requirement for proposal voting.

Require a minimum period to cast a vote, measured in number of blocks, before `close_proposal` can be executed. 

#### Status

:heavy_check_mark: Resolved.

### IS-15 Multiple Proposal Execution in governance

- Location:
  - `governance/governance/src/lib.rs: 106-129`

An approved proposal may be executed multiple times, by invoking several times the `close_proposal` function.

#### Recommendation

After closing a proposal, remove it.

#### Status

:heavy_check_mark: Resolved.

### IS-16 Proposal Leak in governance

- Location:
  - `governance/governance/src/lib.rs`

All the proposals' state is stored in the instance state, which is capped at 64Kb. This issue is made worse by the fact that the data of all the closed proposals are kept in the storage stored in the instance store.

This issue allows a single account to completely and permanently halt the operation of the contract forever by submitting enough proposals.

#### Recommendation

Keep all the data for each proposal on its own persistent entry. This includes, at least, the data in the `votes`, `voting_count `and `proposals` maps.

#### Status

:heavy_check_mark: Resolved

### IS-17 Unrestricted initialize

- Location:
  - `governance/governance/src/lib.rs: 51-66`
  - `governance/mock-contract/src/lib.rs: 19-23`
  - `multisig/src/lib.rs: 37-50`
  - `payment-channel/src/lib.rs: 37-49`

The initialization functions of these contracts can be run at any time by any account effectively removing all the stored state for the instance.

#### Recommendation

Ensure that initialization functions can be run only once.

#### Status

:heavy_check_mark: Resolved.

### IS-18A amm Swap State in instance

- Location:
  - `amm/src/lib.rs: 37, 122-137, 152-154, 208-210`

Adding enough swap pairs may exhaust the 64Kb of instance storage. 

Also keeping a long set of swap pairs, even if not used, will increase the cost of using the amm contract, as the rent for the storage needs to be payed for all the swap pairs

#### Recommendation

Store the state for each swap pair on its own persistent storage slot.

#### Status

:x: Invalid. Each swap pair is already stored in its own key.

### IS-18B amm Swap Should Fail if not Enough Funds

- Location:
  - `amm/src/lib.rs: 201`

The swap function changes the input to the available funds in the from account with this code

```rust
let input = input.min(client_a.balance(&from));
```

This is not a good idea, as it will make the account do an operation different from what it requested.

#### Recommendation

Fail when not enough funds are available.

#### Status

:heavy_check_mark: Resolved.

### IS-19 Constant Sum Considered Harmful

- Location:
  - `amm/csamm`

The amm contract has the option to use constant sum, instead of constant product, to do the exchange. This is a bad idea as it does not adapt to the market conditions.

#### Recommendation

For a sample amm, just use constant product as invariant.

#### Status

:x: Rejected: A second curve is provided as an example for an AMM contract is configurable curves. It’s up to the admin to select a safe curve, and up to an eventual developer to add better curves, if desired.

## Enhancements

### EN-01 Clean Up Git

There are some files that should not have been committed in the repository, and some other files that should have been ignored, not ignored. This includes files in the `target` and `test_snapshots` directories.

Also see IS-01 Bad Cargo.lock Tracking.

#### Recommendation

1. Remove all unwanted files currently in the repository. We found some files in the `governance/governance/test_snapshots` directory. There may be more lurking in the repository.
2. Either make a single `.gitignore` file for the entire repository or have one file for each first-level directory, depending on if you would like to ease copying the directories to make contracts or not.
3. In those `.gitignore` files, make sure that the `target` and `test_snapshots` directories are ignored. Also include other directories and files as appropriate.

#### Status

:x: Not implemented.

### EN-02 Checked Arithmetic

Checked arithmetic is used in the analyzed contracts, even when the `overflow-checks = true` setting has been made in the `Cargo.toml` files. This leads to code that is more difficult to read without upside.

#### Recommendation

1. Use the `overflow-checks = true` setting where possible.
2. Avoid checked arithmetic if not needed. This means only using it to handle overflows when more than a simple fail is required.
3. If you want to showcase some overflows in the examples, keep them minimal and use the `overflow-checks = false` setting.

#### Status

:x: Not implemented.

### EN-03 Test Running Procedure

There is no clear way to run all the tests, neither for a single directory nor for the entire repository. In the directory where a script was provided to run the tests (`multi-contract-caller`) does not stop when a test fails.

#### Recommendation

Ideally, running `cargo test` should be enough to run all the tests. If not possible, at least provide a script to run all the tests, which should fail if any of the run tests fail.

#### Status

:x: Not implemented.

### EN-04 Rust Versions

There is no clear indication of the required version, yet it requires at least rust 1.76.

#### Recommendation

Document the required rust version.

#### Status

:x: Not implemented.

### EN-05 Mock Contract Import in Wrong Place

- Location:
  - `governance/governance/src/lib.rs:6`

The declaration for the `mock_contract` module can be placed in the governance/governance/src/test.rs file.

#### Recommendation

Move it there. See Appendix - Changes Made to the Workspace for details.

#### Status

:heavy_check_mark: Resolved.

### EN-06 Return Error in cpamm curve_fn

- Location:
  - `amm/cpamm/src/lib.rs: 26`

#### Recommendation

Given that it is not mathematically possible to enter the `output > balance_b` branch of the if statement in the `curve_fn` function, return an error instead of `-1` in that case.

#### Status

:heavy_check_mark: Implemented

### EN-07 amm token Upstream

- Location:
  - `amm/token`

The contract in this directory is taken from the [https://github.com/stellar/soroban-examples](https://github.com/stellar/soroban-examples) git repository, yet it is not documented anywhere.

#### Recommendation

Document the upstream source.

#### Status

:heavy_check_mark: Implemented.

### EN-07 amm liquidity withdrawal

- Location:
  - `amm/src/lib.rs`

The amm contract does not have an operation to withdraw the liquidity provided via the deposit function.

#### Recommendation

Add the missing functionality.

#### Status

:heavy_check_mark: Implemented.

### EN-08 No Gains for Liquidity Providers in amm

- Location:
  - `amm/src/lib.rs`

The amm contract does not have a mechanism to reward providing liquidity to the market maker.

#### Recommendation

Add the missing functionality.

#### Status

:x: Rejected. Out of scope.