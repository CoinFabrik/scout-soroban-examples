#!/bin/bash

# Colors
blue_color="\033[34m"
green_color="\033[32m"
red_color="\033[31m"
reset_color="\033[0m"

check_compilation() {
    local patron="^error: could not compile"
    local contract="$1"
    if grep -q "$patron" test-all.txt; then
        echo -e "${red_color}Could not build $contract ${reset_color}"
    else
        # Si no existe, imprimir "Se pudo compilar" en verde
        echo -e "${green_color}Successfully built $contract ${reset_color}"
    fi
}

check_cargo_test(){
    local contract="$1"
    echo -e "
${blue_color}Compiling and running tests for $contract...${reset_color}
"
cargo test --manifest-path $contract/Cargo.toml >  test-all.txt 2>&1
errorline=$(grep "error: could not compile"  test-all.txt)
if [[ -n "$errorline" ]]; then
    echo -e "${red_color}Error found in $contract. Could not compile${reset_color}" 
    grep -v "^   Compiling" test-all.txt
else
    finish_msg=$(grep -xE '    Finished `test` profile.*'  test-all.txt | sed -e 's/^    //')
    echo -e "${green_color}${finish_msg}${reset_color}"
    grep -xE 'test test::test_.*'  test-all.txt
fi
}

## Soroban contract build to necessary dependencies 
echo -e "

Soroban contract building necessary dependencies for testing...
" 

soroban contract build --manifest-path governance/mock-contract/Cargo.toml >  test-all.txt 2>&1
check_compilation "governance/mock-contract"
soroban contract build --manifest-path multi-contract-caller/storage/Cargo.toml >  test-all.txt 2>&1
check_compilation "multi-contract-caller/storage"
soroban contract build --manifest-path multi-contract-caller/adder/Cargo.toml >  test-all.txt 2>&1
check_compilation "multi-contract-caller/adder"
soroban contract build --manifest-path multi-contract-caller/subber/Cargo.toml >  test-all.txt 2>&1
check_compilation "multi-contract-caller/subber"
soroban contract build --manifest-path amm/token/Cargo.toml >  test-all.txt 2>&1
check_compilation "amm/token"
soroban contract build --manifest-path amm/cpamm/Cargo.toml >  test-all.txt 2>&1
check_compilation "amm/cpamm"
soroban contract build --manifest-path amm/csamm/Cargo.toml >  test-all.txt 2>&1
check_compilation "amm/csamm"


check_cargo_test "multisig"
check_cargo_test "amm"
check_cargo_test "governance/governance"
check_cargo_test "multi-contract-caller/adder"
check_cargo_test "multi-contract-caller/caller"
check_cargo_test "multi-contract-caller/storage"
check_cargo_test "multi-contract-caller/subber"
check_cargo_test "payment-channel"
check_cargo_test "vesting"



rm  test-all.txt

