#!/bin/bash

## Soroban contract build to necessary dependencies 
soroban contract build --manifest-path governance/mock-contract/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path multi-contract-caller/storage/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path multi-contract-caller/adder/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path multi-contract-caller/subber/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path amm/token/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path amm/cpamm/Cargo.toml > /dev/null 2>&1
soroban contract build --manifest-path amm/csamm/Cargo.toml > /dev/null 2>&1

cargo test --manifest-path multisig/Cargo.toml 2> compilacion.txt 
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Multisig. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path amm/Cargo.toml 2> compilacion.txt  
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in amm. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path governance/governance/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Governance. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path governance/mock-contract/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in mock-contract. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path multi-contract-caller/adder/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Adder. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path multi-contract-caller/caller/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Caller. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path multi-contract-caller/storage/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Storage. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path multi-contract-caller/subber/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Subber. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi


cargo test --manifest-path payment-channel/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Payment Channel. Could not compile" 
    cat compilacion.txt
else
    grep -v '^Compile' compilacion.txt
fi

cargo test --manifest-path vesting/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Vesting. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi






