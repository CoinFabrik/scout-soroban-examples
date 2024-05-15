# Governance Mock Contract

## Overview

This contract is utilized by the [governance](https://github.com/CoinFabrik/scout-soroban-examples/tree/main/governance) smart contract for testing purposes. It designates a governance program as its authority in the constructor and contains a `counter` variable that increments by 1 each time `increase_counter()` is called.

In the test, a proposal is created that, upon approval, will trigger the execution of `increase_counter()`.
