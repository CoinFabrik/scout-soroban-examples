#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{symbol_short, token, Address, Env};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

mod cpamm {
    soroban_sdk::contractimport!(
        file = "cpamm/target/wasm32-unknown-unknown/release/cpamm.wasm"
    );
}

mod csamm {
    soroban_sdk::contractimport!(
        file = "csamm/target/wasm32-unknown-unknown/release/csamm.wasm"
    );
}

mod soroban_token_contract {
    soroban_sdk::contractimport!(
        file = "token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}

fn create_cpamm_contract<'a>(e: &Env) -> util::SwapCurveClient<'a> {
    util::SwapCurveClient::new(e, &e.register_contract_wasm(None, cpamm::WASM))
}

fn create_csamm_contract<'a>(e: &Env) -> util::SwapCurveClient<'a> {
    util::SwapCurveClient::new(e, &e.register_contract_wasm(None, csamm::WASM))
}

fn create_contract<'a>(e: &Env) -> SwapContractClient<'a> {
    SwapContractClient::new(e, &e.register_contract(None, SwapContract {}))
}

fn create_token_contract<'a>(e: &Env) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_contract_wasm(None, soroban_token_contract::WASM))
}

fn create_token_pair<'a>(e: &Env, admin: &Address) -> (TokenAdminClient<'a>, TokenClient<'a>){
    let contract = e.register_stellar_asset_contract(admin.clone());
    (TokenAdminClient::new(&e, &contract), TokenClient::new(&e, &contract))
}

#[test]
fn test_happy_path(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let token_hash = env.deployer().upload_contract_wasm(soroban_token_contract::WASM);
    let cpamm = create_cpamm_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token_admin3 = Address::generate(&env);

    let (token_admin_client1, token1) = create_token_pair(&env, &token_admin1);
    let (token_admin_client2, token2) = create_token_pair(&env, &token_admin2);
    let (token_admin_client3, token3) = create_token_pair(&env, &token_admin3);

    let admin = Address::generate(&env);

    contract.initialize(&admin, &token_hash);

    contract.new_swap(&admin, &token1.address, &token2.address, &cpamm.address);

    token_admin_client1.mint(&admin, &1_000_000_000);
    token_admin_client2.mint(&admin, &3_000_000_000);

    contract.deposit(
        &admin,
        &(token1.address.clone(), 1_000_000_000_i128, 1_000_000_000_i128),
        &(token2.address.clone(), 3_000_000_000_i128, 3_000_000_000_i128),
        &admin,
    );

    let alice = Address::generate(&env);

    token_admin_client1.mint(&alice, &1_000);

    let received = contract.swap(&alice, &alice, &token1.address, &token2.address, &1_000, &0);

    assert_eq!(0, token1.balance(&alice));
    assert_eq!(received, token2.balance(&alice));

    let received2 = contract.swap(&alice, &alice, &token2.address, &token1.address, &received, &0);

    assert_eq!(received2, token1.balance(&alice));
    assert_eq!(0, token2.balance(&alice));

    let bob = Address::generate(&env);
    token_admin_client1.mint(&bob, &1_000);
    token_admin_client2.mint(&bob, &10_000);

    let result = contract.try_deposit(
        &bob,
        &(token1.address.clone(), 1_000_i128, 1_000_i128),
        &(token2.address.clone(), 10_000_i128, 10_000_i128),
        &admin,
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap().ok().unwrap(), SwapError::DepositRejected);
}

#[test]
#[should_panic]
fn test_double_init(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let token_hash = env.deployer().upload_contract_wasm(soroban_token_contract::WASM);

    let admin = Address::generate(&env);

    contract.initialize(&admin, &token_hash);
    contract.initialize(&admin, &token_hash);
}

#[test]
#[should_panic]
fn test_no_init_1(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let cpamm = create_cpamm_contract(&env);
    let token = create_token_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);

    let token_admin_client1 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin1));
    let token_admin_client2 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin2));

    let admin = Address::generate(&env);

    contract.new_swap(&admin, &token_admin_client1.address, &token_admin_client2.address, &cpamm.address);
}

#[test]
#[should_panic]
fn test_no_init_2(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let cpamm = create_cpamm_contract(&env);
    let token = create_token_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);

    let token_admin_client1 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin1));
    let token_admin_client2 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin2));

    let from = Address::generate(&env);

    token_admin_client1.mint(&from, &1000);
    token_admin_client2.mint(&from, &1000);

    contract.deposit(
        &from,
        &(token_admin_client1.address, 1_i128, 1_i128),
        &(token_admin_client2.address, 1_i128, 1_i128),
        &from,
    );
}

#[test]
#[should_panic]
fn test_no_init_3(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let cpamm = create_cpamm_contract(&env);
    let token = create_token_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);

    let token_admin_client1 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin1));
    let token_admin_client2 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin2));

    let from = Address::generate(&env);

    token_admin_client1.mint(&from, &1000);

    contract.swap(&from, &from, &token_admin_client1.address, &token_admin_client2.address, &1_000, &1_000);
}

#[test]
fn test_no_init_4(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let cpamm = create_cpamm_contract(&env);
    let token = create_token_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);

    let token_admin_client1 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin1));
    let token_admin_client2 = TokenAdminClient::new(&env, &env.register_stellar_asset_contract(token_admin2));

    let from = Address::generate(&env);

    contract.swap(&from, &from, &token_admin_client1.address, &token_admin_client2.address, &1_000, &1_000);
}

#[test]
#[should_panic]
fn test_bad_swap(){
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_contract(&env);
    let token_hash = env.deployer().upload_contract_wasm(soroban_token_contract::WASM);
    let cpamm = create_cpamm_contract(&env);

    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token_admin3 = Address::generate(&env);

    let (token_admin_client1, token1) = create_token_pair(&env, &token_admin1);
    let (token_admin_client2, token2) = create_token_pair(&env, &token_admin2);
    let (token_admin_client3, token3) = create_token_pair(&env, &token_admin3);

    let admin = Address::generate(&env);

    contract.initialize(&admin, &token_hash);

    contract.new_swap(&admin, &token1.address, &token2.address, &cpamm.address);

    token_admin_client1.mint(&admin, &1_000_000_000);
    token_admin_client2.mint(&admin, &3_000_000_000);

    contract.deposit(
        &admin,
        &(token1.address.clone(), 1_000_000_000_i128, 1_000_000_000_i128),
        &(token2.address.clone(), 3_000_000_000_i128, 3_000_000_000_i128),
        &admin,
    );

    let alice = Address::generate(&env);

    token_admin_client1.mint(&alice, &1_000);

    let received = contract.swap(&alice, &alice, &token1.address, &token3.address, &1_000, &0);

}
