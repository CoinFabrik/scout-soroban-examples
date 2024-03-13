#![cfg(test)]
extern crate std; 

use core::ops::Add;
use std::os::linux::raw::stat;
use std::println;

use super::*; 
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger}; 
use soroban_sdk::{symbol_short, token, vec, Address, Env, IntoVal}; 
use token::Client as TokenClient; 
use token::StellarAssetClient as TokenAdminClient; 


#[test]
fn test_initialize_a_multisig() {
    let env = Env::default(); 
    env.mock_all_auths(); 

    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let owner3 = Address::generate(&env);
    let owner4 = Address::generate(&env);
    let not_owner = Address::generate(&env);

    let owners_vec = vec![&env, owner1.clone(), owner2.clone(), owner3.clone(), owner4.clone()]; 
    let multisig_contract = MultisigClient::new(&env, &env.register_contract(None, Multisig{}));

    multisig_contract.initialize_multisig(&owners_vec, &3); 

    let mut state = multisig_contract.get_state(); 

    assert_eq!(state.owners, owners_vec);
    assert_eq!(state.required_signatures, 3); 


    assert!(multisig_contract.is_owner(&owner1)); 
    assert!(multisig_contract.is_owner(&owner2)); 
    assert!(multisig_contract.is_owner(&owner3)); 
    assert!(multisig_contract.is_owner(&owner4)); 
    assert!(!multisig_contract.is_owner(&not_owner)); 

    let token_admin = Address::generate(&env);

    let contract_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &contract_address);
    let token_admin_client = TokenAdminClient::new(&env, &contract_address);
    token_admin_client.mint(&multisig_contract.address, &1_000_000_000_000);

    assert_eq!(token.balance(&multisig_contract.address), 1_000_000_000_000);
    let proposed_amount = 20_000; 
    let proposed_receiver = not_owner.clone(); 


    multisig_contract.submit_tx(&token.address, &proposed_receiver, &proposed_amount, &owner2); 

    assert_eq!(
        env.auths(),
        std::vec![
            (
                owner2.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        multisig_contract.address.clone(),
                        symbol_short!("submit_tx"),
                        (&token.address, proposed_receiver.clone(), proposed_amount, owner2.clone()).into_val(&env)
                    )),
                    sub_invocations: std::vec![]
                },
            )
        ]
    );

    let proposed_tx = ProposedTx {
        token: token.address.clone(), 
        tx_id: 0, 
        transfer_to: proposed_receiver.clone(), 
        transfer_amount: proposed_amount
    };
    state = multisig_contract.get_state(); 
    assert_eq!(state.next_tx_id, 1); 
    assert_eq!(state.transactions.get(0).unwrap(), proposed_tx);
    
    multisig_contract.confirm_transaction(&0, &owner1); 
    state = multisig_contract.get_state(); 

    assert_eq!(state.confirmation_count.get(0).unwrap(), 1); 
    assert_eq!(state.confirmations.get((0, owner1)).unwrap(), ()); 

    multisig_contract.confirm_transaction(&0, &owner2);
    multisig_contract.confirm_transaction(&0, &owner3);

    assert_eq!(token.balance(&not_owner), 0);
    multisig_contract.execute_transaction(&0); 

    
    assert_eq!(token.balance(&multisig_contract.address), 999_999_980_000);
    assert_eq!(token.balance(&not_owner), 20_000); 

    
}
