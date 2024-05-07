#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{symbol_short, token, vec, Address, Env, IntoVal};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

#[test]
fn test_initialize_a_multisig() {
    let env = Env::default();
    env.mock_all_auths();

    // initialize owners and other actors
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let owner3 = Address::generate(&env);
    let owner4 = Address::generate(&env);
    let not_owner = Address::generate(&env);

    let owners_vec = vec![
        &env,
        owner1.clone(),
        owner2.clone(),
        owner3.clone(),
        owner4.clone(),
    ];
    let multisig_contract = MultisigClient::new(&env, &env.register_contract(None, Multisig {}));

    // create a token for testing purposes
    let token_admin = Address::generate(&env);
    let contract_address = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &contract_address);
    let token_admin_client = TokenAdminClient::new(&env, &contract_address);

    // INITIALIZE MULTISIG
    multisig_contract.initialize_multisig(&owners_vec, &3);

    let mut state = multisig_contract.get_multisig_state();

    assert_eq!(state.owners, owners_vec);
    assert_eq!(state.required_signatures, 3);

    // verify owners
    assert!(multisig_contract.is_owner(&owner1));
    assert!(multisig_contract.is_owner(&owner2));
    assert!(multisig_contract.is_owner(&owner3));
    assert!(multisig_contract.is_owner(&owner4));
    assert!(!multisig_contract.is_owner(&not_owner));

    // mint tokens to the contract for testing purposes
    token_admin_client.mint(&multisig_contract.address, &1_000_000_000_000);
    assert_eq!(token.balance(&multisig_contract.address), 1_000_000_000_000);

    // CREATE A TRANSACTION PROPOSAL
    let proposed_amount = 20_000;
    let proposed_receiver = not_owner.clone();

    multisig_contract.submit_tx(
        &token.address,
        &proposed_receiver,
        &proposed_amount,
        &owner2,
    );

    // verify authorities
    assert_eq!(
        env.auths(),
        std::vec![(
            owner2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    multisig_contract.address.clone(),
                    symbol_short!("submit_tx"),
                    (
                        &token.address,
                        proposed_receiver.clone(),
                        proposed_amount,
                        owner2.clone()
                    )
                        .into_val(&env)
                )),
                sub_invocations: std::vec![]
            },
        )]
    );

    let proposed_tx = ProposedTx {
        confirmation_count: 0,
        token: token.address.clone(),
        tx_id: 0,
        transfer_to: proposed_receiver.clone(),
        transfer_amount: proposed_amount,
        executed: false,
    };
    state = multisig_contract.get_multisig_state();
    assert_eq!(state.next_tx_id, 1);
    assert_eq!(multisig_contract.get_proposed_tx(&0), proposed_tx);

    // CONFIRM A TRANSACTION
    multisig_contract.confirm_transaction(&0, &owner1);

    let proposal_state = multisig_contract.get_proposed_tx(&0);
    assert_eq!(proposal_state.confirmation_count, 1);
    assert!(multisig_contract.get_confirmation(&0, &owner1).confirmed);

    multisig_contract.confirm_transaction(&0, &owner2);
    multisig_contract.confirm_transaction(&0, &owner3); // reached 3 signatures -> previously set as required signatures

    // EXECUTE AN ALREADY APPROVED TRANSACTION
    assert_eq!(token.balance(&not_owner), 0);
    multisig_contract.execute_transaction(&0);

    assert_eq!(token.balance(&multisig_contract.address), 999_999_980_000);
    assert_eq!(token.balance(&not_owner), 20_000);

    // OWNER ADDITION PROPOSAL - try to add `not_owner`

    // confirmation 1
    multisig_contract.approve_owner_addition(&not_owner, &owner1);
    let mut modification_proposal = multisig_contract.get_member_modification(&not_owner);

    assert!(
        multisig_contract
            .get_member_confirmation(&modification_proposal.modification_id, &owner1)
            .active
    );
    assert_eq!(modification_proposal.confirmation_count, 1);

    // confirmation 2
    multisig_contract.approve_owner_addition(&not_owner, &owner2);
    modification_proposal = multisig_contract.get_member_modification(&not_owner);
    assert!(
        multisig_contract
            .get_member_confirmation(&modification_proposal.modification_id, &owner2)
            .active
    );
    assert_eq!(modification_proposal.confirmation_count, 2);

    assert!(!multisig_contract.is_owner(&not_owner));

    // confirmation 3  -> reached required signatures
    multisig_contract.approve_owner_addition(&not_owner, &owner3);
    assert!(multisig_contract.is_owner(&not_owner));

    // OWNER REMOVAL PROPOSAL - try to remove `owner_3`

    // confirmation 1
    multisig_contract.approve_owner_removal(&owner3, &owner1);

    modification_proposal = multisig_contract.get_member_modification(&owner3);
    assert_eq!(modification_proposal.modification_id, 1);
    assert!(
        multisig_contract
            .get_member_confirmation(&modification_proposal.modification_id, &owner1)
            .active
    );
    assert_eq!(modification_proposal.confirmation_count, 1);

    // confirmation 2
    multisig_contract.approve_owner_removal(&owner3, &owner2);

    modification_proposal = multisig_contract.get_member_modification(&owner3);
    assert!(
        multisig_contract
            .get_member_confirmation(&modification_proposal.modification_id, &owner2)
            .active
    );
    assert_eq!(modification_proposal.confirmation_count, 2);

    assert!(multisig_contract.is_owner(&owner3));

    // confirmation 3 -> reached required signatures
    multisig_contract.approve_owner_removal(&owner3, &owner4);

    assert!(!multisig_contract.is_owner(&owner3));
}
