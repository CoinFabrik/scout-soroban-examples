#![cfg(test)]
extern crate std; 

use crate::mock_contract;

use super::*; 
use soroban_sdk::testutils::{Address as _, Ledger}; 
use soroban_sdk::{Address, Env}; 


#[test]
fn test_governance_supermajority_false() {

    
    let env = Env::default(); 
    let governance_contract = GovernanceClient::new(&env, &env.register_contract(None, Governance{}));
    

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

    // SET BLOCK TIMESTAMP - DATE March 13 17:00:00 
    env.ledger().with_mut(|info|{
        info.timestamp += 1710360000;
    });

    // INITIALIZE GOVERNANCE 
    let one_week_seconds = 604800; 
    let supermajority = false; 
    let voting_period = one_week_seconds; 
    
    governance_contract.initialize(&supermajority, &None, &voting_period); 

    let mut governance_state = governance_contract.get_state(); 

    //verify initialization
    assert_eq!(governance_state.supermajority, false); 
    assert_eq!(governance_state.supermajority_percentage, None); 
    assert_eq!(governance_state.voting_period, one_week_seconds); 
    assert_eq!(governance_state.votes, Map::new(&env)); 
    assert_eq!(governance_state.voting_count, Map::new(&env)); 
    assert_eq!(governance_state.proposals, Map::new(&env)); 
    assert_eq!(governance_state.next_tx_id, 0);

    // CREATE A PROPOSAL
    let period_fn = Symbol::new(&env, "increase_counter");
    governance_contract.propose_tx(&&mock_contract_address, &Symbol::new(&env, "increase_counter"), &Vec::new(&env));

    governance_state = governance_contract.get_state(); 
    assert_eq!(governance_state.next_tx_id, 1);
    assert_eq!(governance_state.proposals.len(), 1);  

    // verify proposal 
    let proposal_state = governance_contract.get_proposal(&0); 
    assert_eq!(proposal_state.proposal_id, 0); 
    assert_eq!(proposal_state.contract_id, mock_contract_address); 
    assert_eq!(proposal_state.function, period_fn); 
    assert_eq!(proposal_state.func_arguments, Vec::new(&env)); 
    assert_eq!(proposal_state.expiration_date, env.ledger().timestamp() + one_week_seconds); 

    // VOTE ON A PROPOSAL
    let mut proposal_id = 0; 
    governance_contract.vote_proposal(&voter1, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter2, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter3, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter4, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter5, &proposal_id, &false); 
    governance_contract.vote_proposal(&voter6, &proposal_id, &false); 

    governance_state = governance_contract.get_state(); 

    assert_eq!(governance_state.voting_count.get((proposal_id, true)).unwrap_or(0), 4); 
    assert_eq!(governance_state.voting_count.get((proposal_id, false)).unwrap_or(0), 2);
    assert_eq!(governance_state.votes.get((proposal_id, voter1.clone())).unwrap(), true);
    assert_eq!(governance_state.votes.get((proposal_id, voter2.clone())).unwrap(), true);
    assert_eq!(governance_state.votes.get((proposal_id, voter3.clone())).unwrap(), true);
    assert_eq!(governance_state.votes.get((proposal_id, voter4.clone())).unwrap(), true);
    assert_eq!(governance_state.votes.get((proposal_id, voter5.clone())).unwrap(), false);
    assert_eq!(governance_state.votes.get((proposal_id, voter6.clone())).unwrap(), false);


    // CLOSE PROPOSAL 

    //move time to be in the future -- 10 days 
    env.ledger().with_mut(|info|{
        info.timestamp += 864000;
    });

    // verify mock program state before closing the proposal
    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);  

    governance_contract.close_proposal(&proposal_id); 

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1);  



    // Create proposal 2 == proposal 1 to verify negative result 

    governance_contract.propose_tx(&&mock_contract_address, &Symbol::new(&env, "increase_counter"), &Vec::new(&env));
    proposal_id = 1; 
    governance_contract.vote_proposal(&voter1, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter2, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter3, &proposal_id, &false); 
    governance_contract.vote_proposal(&voter4, &proposal_id, &false); 
    governance_contract.vote_proposal(&voter5, &proposal_id, &false); 
    governance_contract.vote_proposal(&voter6, &proposal_id, &false); 

    env.ledger().with_mut(|info|{
        info.timestamp += 864000;
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
    let governance_contract = GovernanceClient::new(&env, &env.register_contract(None, Governance{}));
    

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

    // SET BLOCK TIMESTAMP - DATE March 13 17:00:00 
    env.ledger().with_mut(|info|{
        info.timestamp += 1710360000;
    });

    // INITIALIZE GOVERNANCE 
    let one_week_seconds = 604800; 
    let supermajority = true; 
    let voting_period = one_week_seconds; 
    
    governance_contract.initialize(&supermajority, &Some(80), &voting_period); 

    // CREATE A PROPOSAL
    governance_contract.propose_tx(&&mock_contract_address, &Symbol::new(&env, "increase_counter"), &Vec::new(&env));
    
    // VOTE ON A PROPOSAL
    let mut proposal_id = 0; 
    governance_contract.vote_proposal(&voter1, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter2, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter3, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter4, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter5, &proposal_id, &false); 
    governance_contract.vote_proposal(&voter6, &proposal_id, &false); 

    // CLOSE PROPOSAL 

    //move time to be in the future -- 10 days 
    env.ledger().with_mut(|info|{
        info.timestamp += 864000;
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

    governance_contract.propose_tx(&&mock_contract_address, &Symbol::new(&env, "increase_counter"), &Vec::new(&env));
    proposal_id = 1; 
    governance_contract.vote_proposal(&voter1, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter2, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter3, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter4, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter5, &proposal_id, &true); 
    governance_contract.vote_proposal(&voter6, &proposal_id, &false); 

    env.ledger().with_mut(|info|{
        info.timestamp += 864000;
    });

    let mut mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 0);  

    governance_contract.close_proposal(&proposal_id); 

    //verify after
    mock_state = mock_contract_client.get_state();
    assert_eq!(mock_state.counter, 1); 

}