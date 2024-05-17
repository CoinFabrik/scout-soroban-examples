#!/bin/bash

cargo clean --manifest-path governance/mock-contract/Cargo.toml  
cargo clean --manifest-path governance/governance/Cargo.toml  
cargo clean --manifest-path multisig/Cargo.toml  
cargo clean --manifest-path multi-contract-caller/storage/Cargo.toml  
cargo clean --manifest-path multi-contract-caller/adder/Cargo.toml  
cargo clean --manifest-path multi-contract-caller/subber/Cargo.toml  
cargo clean --manifest-path multi-contract-caller/caller/Cargo.toml  
cargo clean --manifest-path amm/Cargo.toml  
cargo clean --manifest-path amm/token/Cargo.toml  
cargo clean --manifest-path amm/cpamm/Cargo.toml  
cargo clean --manifest-path amm/csamm/Cargo.toml  
cargo clean --manifest-path payment-channel/Cargo.toml  
cargo clean --manifest-path vesting/Cargo.toml  