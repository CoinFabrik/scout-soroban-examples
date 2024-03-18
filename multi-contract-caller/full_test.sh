#!/bin/sh
cd storage
soroban contract build
cargo test
cd ../adder
soroban contract build
cargo test
cd ../subber
soroban contract build
cargo test
cd ../caller
soroban contract build
cargo test
