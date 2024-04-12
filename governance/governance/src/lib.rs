#![no_std]

use soroban_sdk::{contracterror, contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol, Val, Vec}; 

// Uncomment this in order to test 
mod mock_contract {
    soroban_sdk::contractimport!(
        file = "../mock-contract/target/wasm32-unknown-unknown/release/mock_contract.wasm"
    );
}

#[derive(Debug, Clone, PartialEq)]
#[contracttype]
pub struct ProposedTx {
    pub proposal_id: u32,           // The ID of the proposal.
    pub contract_id: Address,       // The address of the contract the tx will invoke if approved.
    pub function: Symbol,           // The function to be invoked if approved.
    pub func_arguments: Vec<Val>,   // The parameters to the function. 
    pub expiration_date: u64,       // The timestamp when the proposal can no longer be voted on. 
}

#[contracttype]
#[derive(Debug)]
pub struct GovernanceState {
    votes: Map<(u32, Address), bool>,       // Map<(ProposalId, VoterAddress), VoteValue> - Keeps record of users' votes. 
    voting_count: Map<(u32, bool), u32>,    // Map<(ProposalId, VotesValue), AmountOfVotes> - Keeps record of all negative and positive votes for a proposal. 
    proposals: Map<u32, ProposedTx>,        // Map<ProposalId, Proposal> - Keeps record of the proposals. 
    next_tx_id: u32,                        // Next proposal id to be assigned. 
    supermajority: bool,                    // Indicates whether the amount of positive votes has to be higher than 50%. 
    supermajority_percentage: Option<u32>,  // Indicates the minimum percentage the amount of positive votes has to reach. For example, 80% over total votes. 
    voting_period: u64                      // Indicates how much time the voting of a proposal remains open from the moment of its creation. 
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GovError {
    InvalidSupermajorityPercentage = 1, 
}


const GOVERNANCE_STATE: Symbol = symbol_short!("GOV_STATE"); 


#[contract]
pub struct Governance; 

#[contractimpl]
impl Governance {

    pub fn initialize(env: Env, supermajority: bool, supermajority_percentage: Option<u32>, voting_period: u64) -> Result<(), GovError> {
        if supermajority {
            match supermajority_percentage {
                Some(x) => assert!(x <= 100 && x > 50), 
                None => return Err(GovError::InvalidSupermajorityPercentage),
            }
        }
        let new_gov_state = GovernanceState {
            votes: Map::new(&env), 
            voting_count: Map::new(&env), 
            proposals: Map::new(&env), 
            next_tx_id: 0, 
            supermajority, 
            supermajority_percentage,
            voting_period
        }; 

        env.storage().instance().set(&GOVERNANCE_STATE, &new_gov_state);
        Ok(())
    }

    pub fn propose_tx(env: Env, contract_id: Address, func_name: Symbol, func_args: Vec<Val>){
        let mut state = Self::get_state(env.clone()); 
        let now = env.ledger().timestamp(); 
        let proposal_id = state.next_tx_id; 
        state.next_tx_id += 1; 
        let new_proposal = ProposedTx {
            proposal_id,
            contract_id, 
            function: func_name, 
            func_arguments: func_args, 
            expiration_date: now.checked_add(state.voting_period).unwrap()
        };
        state.proposals.set(proposal_id, new_proposal); 
        env.storage().instance().set(&GOVERNANCE_STATE, &state);
    }

    pub fn vote_proposal(env: Env, voter: Address, proposal_id: u32, vote_value: bool){
        let mut state = Self::get_state(env.clone()); 
        let proposal = state.proposals.get(proposal_id).unwrap(); 
        let now = env.ledger().timestamp(); 

        assert!(now <= proposal.expiration_date); 
        assert!(!state.votes.contains_key((proposal_id, voter.clone()))); 

        state.votes.set((proposal_id, voter.clone()), vote_value); 
        let mut voting_count = state.voting_count.get((proposal_id, vote_value)).unwrap_or(0); 
        voting_count += 1; 

        state.voting_count.set((proposal_id, vote_value), voting_count); 

        env.storage().instance().set(&GOVERNANCE_STATE, &state);

    }

    pub fn close_proposal(env: Env, proposal_id: u32){
        let state = Self::get_state(env.clone()); 
        let proposal = state.proposals.get(proposal_id).unwrap();
        assert!(proposal.expiration_date < env.ledger().timestamp());

        let positive_votes = state.voting_count.get((proposal_id, true)).unwrap_or(0); 
        let negative_votes = state.voting_count.get((proposal_id, false)).unwrap_or(0);  

        match state.supermajority {
            true => {
                let percentage = state.supermajority_percentage.unwrap();
                let total_votes = positive_votes.checked_add(negative_votes).unwrap();  
                if positive_votes*100 >= (total_votes*percentage) {
                    let _res: Val = env.invoke_contract(&proposal.contract_id, &proposal.function, proposal.func_arguments);
                }
            }, 
            false => {
                if positive_votes > negative_votes {
                    let _res: Val = env.invoke_contract(&proposal.contract_id, &proposal.function, proposal.func_arguments);
                }
            }
        }; 

    }

    pub fn get_state(env: Env) -> GovernanceState {
        env.storage().instance().get(&GOVERNANCE_STATE).unwrap()
    }

    pub fn get_proposal(env: Env, proposal_id: u32) -> ProposedTx {
        let state = Self::get_state(env); 
        state.proposals.get(proposal_id).unwrap()
    }


}


#[cfg(test)]
mod test;
