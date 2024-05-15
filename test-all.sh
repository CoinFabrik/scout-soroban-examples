#!/bin/bash

## Soroban contract build to necessary dependencies 
soroban contract build --manifest-path governance/mock-contract/Cargo.toml
soroban contract build --manifest-path multi-contract-caller/storage/Cargo.toml
soroban contract build --manifest-path multi-contract-caller/adder/Cargo.toml
soroban contract build --manifest-path multi-contract-caller/subber/Cargo.toml
soroban contract build --manifest-path amm/token/Cargo.toml


echo "Compiling multisig..." 
cargo build --manifest-path multisig/Cargo.toml 2> compilacion.txt 
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Multisig. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling amm..." 
cargo build --manifest-path amm/Cargo.toml 2> compilacion.txt  
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in amm. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling governance..." 
cargo build --manifest-path governance/governance/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Governance. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling mock-contract for governance..." 
cargo build --manifest-path governance/mock-contract/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in mock-contract. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling multi-contract-caller adder..." 
cargo build --manifest-path multi-contract-caller/adder/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Adder. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling multi-contract-caller caller..." 
cargo build --manifest-path multi-contract-caller/caller/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Caller. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling multi-contract-caller storage..." 
cargo build --manifest-path multi-contract-caller/storage/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Storage. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling multi-contract-caller subber..." 
cargo build --manifest-path multi-contract-caller/subber/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Subber. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

echo "Compiling payment-channel..." 
cargo build --manifest-path payment-channel/Cargo.toml 2> compilacion.txt
errorline=$(grep "error: could not compile" compilacion.txt)
if [[ -n "$errorline" ]]; then
    echo "Error found in Payment Channel. Could not compile" 
    cat compilacion.txt
else
    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
    if [[ -n "$finished_line" ]]; then
        echo -e "\e[32m$finished_line\e[0m"
    fi
fi

#echo "Compiling vesting..." 
#cargo build --manifest-path vesting/Cargo.toml 2> compilacion.txt
#errorline=$(grep "error: could not compile" compilacion.txt)
#if [[ -n "$errorline" ]]; then
#    echo "Error found in Vesting. Could not compile" 
#    cat compilacion.txt
#else
#    finished_line=$(grep "Finished \`dev\`" compilacion.txt)
#    if [[ -n "$finished_line" ]]; then
#        echo -e "\e[32m$finished_line\e[0m"
#    fi
#fi




cargo test --manifest-path multisig/Cargo.toml
cargo test --manifest-path amm/Cargo.toml
cargo test --manifest-path governance/governance/Cargo.toml
cargo test --manifest-path governance/mock-contract/Cargo.toml
cargo test --manifest-path multi-contract-caller/adder/Cargo.toml
cargo test --manifest-path multi-contract-caller/caller/Cargo.toml
cargo test --manifest-path multi-contract-caller/storage/Cargo.toml
cargo test --manifest-path multi-contract-caller/subber/Cargo.toml
cargo test --manifest-path payment-channel/Cargo.toml
# cargo test --manifest-path vesting/Cargo.toml


