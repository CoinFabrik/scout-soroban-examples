#![cfg(test)]
extern crate std;
use super::*;

mod storage_contract {
    soroban_sdk::contractimport!(
        file = "../storage/target/wasm32-unknown-unknown/release/storage_contract.wasm"
    );
}

struct StorageInterfaceSpec;

#[contractspecfn(name = "StorageInterfaceSpec", export = false)]
#[contractclient(crate_path = "crate", name = "StorageClient")]
pub trait StorageInterface {
    fn get(env: Env) -> i64;
    fn set(env: Env, value: i64) -> ();
}

mod adder_contract {
    soroban_sdk::contractimport!(
        file = "../adder/target/wasm32-unknown-unknown/release/adder_contract.wasm"
    );
}

mod subber_contract {
    soroban_sdk::contractimport!(
        file = "../subber/target/wasm32-unknown-unknown/release/subber_contract.wasm"
    );
}

fn create_storage_contract<'a>(e: &Env) -> StorageClient<'a> {
    StorageClient::new(e, &e.register_contract_wasm(None, storage_contract::WASM))
}

fn create_adder_contract<'a>(e: &Env) -> DoerClient<'a> {
    DoerClient::new(e, &e.register_contract_wasm(None, adder_contract::WASM))
}

fn create_subber_contract<'a>(e: &Env) -> DoerClient<'a> {
    DoerClient::new(e, &e.register_contract_wasm(None, subber_contract::WASM))
}

fn create_contract<'a>(e: &Env) -> CallerContractClient<'a> {
    CallerContractClient::new(e, &e.register_contract(None, CallerContract {}))
}

#[test]
fn test_happy_path(){
    let env = Env::default();
    env.mock_all_auths();

    let storage = create_storage_contract(&env);
    let adder = create_adder_contract(&env);
    let subber = create_subber_contract(&env);
    let caller = create_contract(&env);

    caller.init(&storage.address, &adder.address, &subber.address);
    let result = caller.variable_do_it(&42);
    assert_eq!(result, storage.get());
    assert_eq!(result, 42);
    
    let result = caller.variable_do_it(&41);
    assert_eq!(result, storage.get());
    assert_eq!(result, 83);

    caller.flip();

    let result = caller.variable_do_it(&3);
    assert_eq!(result, storage.get());
    assert_eq!(result, 80);

    caller.flip();

    let result = caller.variable_do_it(&20);
    assert_eq!(result, storage.get());
    assert_eq!(result, 100);

    let result = caller.variable_do_it(&0x7FFFFFFF_FFFFFFFF_i64);
    assert_eq!(result, storage.get());
    assert_eq!(result, 0x7FFFFFFF_FFFFFFFF_i64);
    
    let result = caller.variable_do_it(&1);
    assert_eq!(result, storage.get());
    assert_eq!(result, 0x7FFFFFFF_FFFFFFFF_i64);
}

#[test]
#[should_panic]
fn test_double_init(){
    let env = Env::default();
    env.mock_all_auths();

    let storage = create_storage_contract(&env);
    let adder = create_adder_contract(&env);
    let subber = create_subber_contract(&env);
    let caller = create_contract(&env);

    caller.init(&storage.address, &adder.address, &subber.address);
    caller.init(&storage.address, &adder.address, &subber.address);
}

#[test]
#[should_panic]
fn test_uninit2(){
    let env = Env::default();
    env.mock_all_auths();

    let caller = create_contract(&env);

    caller.flip();
}

#[test]
#[should_panic]
fn test_uninit1(){
    let env = Env::default();
    env.mock_all_auths();

    let caller = create_contract(&env);

    caller.variable_do_it(&1);
}

#[test]
fn test_inverted_initialization(){
    let env = Env::default();
    env.mock_all_auths();

    let storage = create_storage_contract(&env);
    let adder = create_adder_contract(&env);
    let subber = create_subber_contract(&env);
    let caller = create_contract(&env);

    caller.init(&storage.address, &subber.address, &adder.address);
    let result = caller.variable_do_it(&42);
    assert_eq!(result, storage.get());
    assert_eq!(result, -42);
    
    let result = caller.variable_do_it(&41);
    assert_eq!(result, storage.get());
    assert_eq!(result, -83);

    caller.flip();

    let result = caller.variable_do_it(&3);
    assert_eq!(result, storage.get());
    assert_eq!(result, -80);

    caller.flip();

    let result = caller.variable_do_it(&20);
    assert_eq!(result, storage.get());
    assert_eq!(result, -100);

    let result = caller.variable_do_it(&0x7FFFFFFF_FFFFFFFF_i64);
    assert_eq!(result, storage.get());
    assert_eq!(result, -0x80000000_00000000_i64);
    
    let result = caller.variable_do_it(&1);
    assert_eq!(result, storage.get());
    assert_eq!(result, -0x80000000_00000000_i64);
}

