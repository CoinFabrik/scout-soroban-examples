#![cfg(test)]
extern crate std;
use super::*;

mod storage_contract {
    soroban_sdk::contractimport!(
        file = "../1storage/target/wasm32-unknown-unknown/release/storage_contract.wasm"
    );
}

fn create_storage_contract<'a>(e: &Env) -> StorageClient<'a> {
    StorageClient::new(e, &e.register_contract_wasm(None, storage_contract::WASM))
}

fn create_contract<'a>(e: &Env) -> AdderContractClient<'a> {
    AdderContractClient::new(e, &e.register_contract(None, AdderContract {}))
}

#[test]
fn test_creation(){
    let env = Env::default();
    env.mock_all_auths();

    let storage = create_storage_contract(&env);
    let adder = create_contract(&env);

    assert_eq!(storage.get(), 0);
    let result = adder.do_it(&storage.address, &1);
    assert_eq!(storage.get(), result);
    assert_eq!(result, 1);
    let result = adder.do_it(&storage.address, &42);
    assert_eq!(storage.get(), result);
    assert_eq!(result, 43);
}
