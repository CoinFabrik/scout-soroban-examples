#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Map, Symbol, Vec, Val}; 

#[derive(Debug, Clone, PartialEq)]
#[contracttype]
pub struct ProposedTx {
    pub proposal_id: u32, 
    pub contract_id: Address, 
    pub function: Symbol,
    pub func_arguments: Vec<Val>, 
    pub expiration_date: u64, 
}

#[contracttype]
#[derive(Debug)]
pub struct GovernanceState {
    votes: Map<(u32 /*proposal_id*/, Address), bool>, 
    voting_count: Map<(u32 /*proposal_id*/, bool), u32>, 
    proposals: Map<u32 /*proposal_id*/, ProposedTx>, 
    next_tx_id: u32,
    supermajority: bool, 
    supermajority_percentage: Option<u32>, 
    voting_period: u64
}

const GOVERNANCE_STATE: Symbol = symbol_short!("GOV_STATE"); 

#[contract]
pub struct Governance; 

#[contractimpl]
impl Governance {

    pub fn initialize(env: Env, supermajority: bool, supermajority_percentage: Option<u32>, voting_period: u64) -> GovernanceState {
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
        Self::get_state(env)
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
        assert!(proposal.expiration_date > env.ledger().timestamp());

        let positive_votes = state.voting_count.get((proposal_id, true)).unwrap_or(0); 
        let negative_votes = state.voting_count.get((proposal_id, false)).unwrap_or(0);  

        match state.supermajority {
            true => {
                let percentage = state.supermajority_percentage.unwrap();
                let total_votes = positive_votes.checked_add(negative_votes).unwrap();  
                if positive_votes >= (total_votes*percentage/100) {
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


}
