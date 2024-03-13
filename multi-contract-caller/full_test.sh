#!/bin/sh
cd 1storage
soroban contract build
cargo test
cd ../2adder
soroban contract build
cargo test
cd ../3subber
soroban contract build
cargo test
cd ../4caller
soroban contract build
cargo test
