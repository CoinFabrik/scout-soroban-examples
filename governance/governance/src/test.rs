#![cfg(test)]
extern crate std;

mod mock_contract {
    soroban_sdk::contractimport!(
        file = "../mock-contract/target/wasm32-unknown-unknown/release/mock_contract.wasm"
    );
}

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, Env};

#[test]
fn test_governance_supermajority_false() {
    let env = Env::default();
    let governance_contract =
        GovernanceClient::new(&env, &env.register_contract(None, Governance {}));

    // initialize a mock contract whose authority will be the governance contract
    let mock_contract_address = env.register_contract_wasm(None, mock_contract::WASM);
    let mock_contract_client = mock_contract::Client::new(&env, &mock_contract_address);
    mock_contract_client.initialize(&governance_contract.address);

    env.mock_all_auths();

    // initialize necessary actors
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);
    let voter4 = Address::generate(&env);
    let voter5 = Address::generate(&env);
    let voter6 = Address::generate(&env);

    // SET BLOCK SEQUENCE
    env.ledger().with_mut(|info| {
        info.sequence_number += 1000;
    });

    // INITIALIZE GOVERNANCE
    let sequence_in_the_future: u32 = 604800;
    let supermajority = false;
    let voting_period = sequence_in_the_future;
    let mut whitelist = Vec::new(&env);
    let quorum: u32 = 50;
    whitelist.push_back(voter1.clone());
    whitelist.push_back(voter2.clone());
    whitelist.push_back(voter3.clone());
    whitelist.push_back(voter4.clone());
    whitelist.push_back(voter5.clone());
    whitelist.push_back(voter6.clone());

    governance_contract.initialize(&supermajority, &None, &voting_period, &whitelist, &quorum);

    let mut governance_state = governance_contract.get_state();

    //verify initialization
    assert_eq!(governance_state.supermajority, false);
    assert_eq!(governance_state.supermajority_percentage, None);
    assert_eq!(governance_state.voting_period, sequence_in_the_future);
    assert_eq!(governance_state.next_tx_id, 0);

    // CREATE A PROPOSAL
    let period_fn = Symbol::new(&env, "increase_counter");
    governance_contract.propose_tx(
        &&mock_contract_address,
        &Symbol::new(&env, "increase_counter"),
        &Vec::new(&env),
        &voter1.clone(),
    );

    governance_state = governance_contract.get_state();
    assert_eq!(governance_state.next_tx_id, 1);

    // verify proposal
    let proposal_state = governance_contract.get_proposal(&0);
    let proposal_tx = proposal_state.tx;
    assert_eq!(
        proposal_state.expiration_date,
        env.ledger().sequence() + sequence_in_the_future
    );
    assert!(!proposal_state.executed);
    assert_eq!(proposal_state.positive_votes, 0);
    assert_eq!(proposal_state.negative_votes, 0);
    assert_eq!(proposal_tx.contract_id, mock_contract_address);
    assert_eq!(proposal_tx.function, period_fn);
    assert_eq!(proposal_tx.func_arguments, Vec::new(&env));

    // VOTE ON A PROPOSAL
    let mut proposal_id = 0;
    governance_contract.vote_proposal(&voter1, &proposal_id, &true);
    governance_contract.vote_proposal(&voter2, &proposal_id, &true);
    governance_contract.vote_proposal(&voter3, &proposal_id, &true);
    governance_contract.vote_proposal(&voter4, &proposal_id, &true);
    governance_contract.vote_proposal(&voter5, &proposal_id, &false);
    governance_contract.vote_proposal(&voter6, &proposal_id, &false);

    let proposal_state = governance_contract.get_proposal(&proposal_id);
    assert_eq!(proposal_state.positive_votes, 4);
    assert_eq!(proposal_state.negative_votes, 2);

    let vote1 = governance_contract.get_vote(&proposal_id, &voter1);
    let vote2 = governance_contract.get_vote(&proposal_id, &voter2);
    let vote3 = governance_contract.get_vote(&proposal_id, &voter3);
    let vote4 = governance_contract.get_vote(&proposal_id, &voter4);
    let vote5 = governance_contract.get_vote(&proposal_id, &voter5);
    let vote6 = governance_contract.get_vote(&proposal_id, &voter6);
    assert!(vote1.active && vote1.value);
    assert!(vote2.active && vote2.value);
    assert!(vote3.active && vote3.value);
    assert!(vote4.active && vote4.value);
    assert!(vote5.active && vote6.active);
    assert!(!(vote5.value && vote6.value));

    // CLOSE PROPOSAL

    //move time to be in the future
    env.ledger().with_mut(|info| {
        info.sequence_number += 864000;
    });

    // verify mock program state before closing the proposal
    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);

    governance_contract.close_proposal(&proposal_id);

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1);

    // Create proposal 2 == proposal 1 to verify negative result

    governance_contract.propose_tx(
        &&mock_contract_address,
        &Symbol::new(&env, "increase_counter"),
        &Vec::new(&env),
        &voter2.clone(),
    );
    proposal_id = 1;
    governance_contract.vote_proposal(&voter1, &proposal_id, &true);
    governance_contract.vote_proposal(&voter2, &proposal_id, &true);
    governance_contract.vote_proposal(&voter3, &proposal_id, &false);
    governance_contract.vote_proposal(&voter4, &proposal_id, &false);
    governance_contract.vote_proposal(&voter5, &proposal_id, &false);
    governance_contract.vote_proposal(&voter6, &proposal_id, &false);

    env.ledger().with_mut(|info| {
        info.sequence_number += 864000;
    });

    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1);

    governance_contract.close_proposal(&proposal_id);

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1);
}

#[test]
fn test_governance_supermajority_true() {
    let env = Env::default();
    let governance_contract =
        GovernanceClient::new(&env, &env.register_contract(None, Governance {}));

    // initialize a mock contract whose authority will be the governance contract
    let mock_contract_address = env.register_contract_wasm(None, mock_contract::WASM);
    let mock_contract_client = mock_contract::Client::new(&env, &mock_contract_address);
    mock_contract_client.initialize(&governance_contract.address);

    env.mock_all_auths();

    // initialize necessary actors
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);
    let voter4 = Address::generate(&env);
    let voter5 = Address::generate(&env);
    let voter6 = Address::generate(&env);

    // SET BLOCK SEQUENCE
    env.ledger().with_mut(|info| {
        info.sequence_number += 1710360000;
    });

    // INITIALIZE GOVERNANCE
    let sequence_in_the_future = 604800;
    let supermajority = true;
    let voting_period = sequence_in_the_future;
    let mut whitelist = Vec::new(&env);
    let quorum: u32 = 50;
    whitelist.push_back(voter1.clone());
    whitelist.push_back(voter2.clone());
    whitelist.push_back(voter3.clone());
    whitelist.push_back(voter4.clone());
    whitelist.push_back(voter5.clone());
    whitelist.push_back(voter6.clone());

    governance_contract.initialize(
        &supermajority,
        &Some(80),
        &voting_period,
        &whitelist,
        &quorum,
    );

    // CREATE A PROPOSAL
    governance_contract.propose_tx(
        &&mock_contract_address,
        &Symbol::new(&env, "increase_counter"),
        &Vec::new(&env),
        &voter3,
    );

    // VOTE ON A PROPOSAL
    let mut proposal_id = 0;
    governance_contract.vote_proposal(&voter1, &proposal_id, &true);
    governance_contract.vote_proposal(&voter2, &proposal_id, &true);
    governance_contract.vote_proposal(&voter3, &proposal_id, &true);
    governance_contract.vote_proposal(&voter4, &proposal_id, &true);
    governance_contract.vote_proposal(&voter5, &proposal_id, &false);
    governance_contract.vote_proposal(&voter6, &proposal_id, &false);

    // CLOSE PROPOSAL

    //move time to be in the future
    env.ledger().with_mut(|info| {
        info.sequence_number += 864000;
    });

    // verify mock program state before closing the proposal
    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);

    /*
    Because positive votes were 4/6 and supermajority requires at least 80%, proposal should not be executed
     */
    governance_contract.close_proposal(&proposal_id);

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);

    // create proposal 2 == proposal 1 to verify positive result

    governance_contract.propose_tx(
        &&mock_contract_address,
        &Symbol::new(&env, "increase_counter"),
        &Vec::new(&env),
        &voter4.clone(),
    );
    proposal_id = 1;
    governance_contract.vote_proposal(&voter1, &proposal_id, &true);
    governance_contract.vote_proposal(&voter2, &proposal_id, &true);
    governance_contract.vote_proposal(&voter3, &proposal_id, &true);
    governance_contract.vote_proposal(&voter4, &proposal_id, &true);
    governance_contract.vote_proposal(&voter5, &proposal_id, &true);
    governance_contract.vote_proposal(&voter6, &proposal_id, &false);

    env.ledger().with_mut(|info| {
        info.sequence_number += 864000;
    });

    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);

    governance_contract.close_proposal(&proposal_id);

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1);
}
