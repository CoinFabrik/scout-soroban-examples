#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, symbol_short, Symbol, Env}; 

#[derive(Debug, Clone)]
#[contracttype]
pub struct State {
    pub governance_auth: Address, 
    pub counter: u32
}

const STATE: Symbol = symbol_short!("MOCKSTATE"); 
#[contract]
pub struct MockProgram; 

#[contractimpl]
impl MockProgram {
    
    pub fn initialize(env: Env, governance_auth: Address) {
        let state = State {
            governance_auth, 
            counter: 0
        };

        env.storage().instance().set(&STATE, &state);
    }

    pub fn increase_counter(env: Env) {
        let mut state = Self::get_state(env.clone()); 
        state.governance_auth.require_auth();
        state.counter += 1; 
        env.storage().instance().set(&STATE, &state);
    }

    pub fn get_state(env: Env) -> State {
        env.storage().instance().get(&STATE).unwrap()
    }
}
