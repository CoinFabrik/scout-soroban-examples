#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, Symbol, Vec};

type TransactionId = u32; 

#[derive(Debug, Clone, PartialEq)]
#[contracttype]
pub struct ProposedTx {
    pub token: Address, 
    pub tx_id: TransactionId,
    pub transfer_to: Address, 
    pub transfer_amount: i128,
}

#[contracttype]
#[derive(Debug)]
pub struct MultisigState {
    confirmations: Map<(TransactionId, Address), ()>, 
    confirmation_count: Map<TransactionId, u32>, 
    transactions: Map<TransactionId, ProposedTx>,
    owners: Vec<Address>, 
    required_signatures: u32,
    next_tx_id: u32,
    pending_additions: Map<Address, u32>, 
    pending_removals: Map<Address, u32>, 
    owner_modifications_conf: Map<(Address, Address, bool), ()>
}

const MULTISIGSTATE: Symbol = symbol_short!("MS_STATE");
#[contract]
pub struct Multisig; 

// soroban attribute that exports the public or usable functions from the contracts 
#[contractimpl]
impl Multisig {
    
    pub fn initialize_multisig(env: Env, owners: Vec<Address>, required_signatures: u32) {
        let new_ms_state = MultisigState {
            confirmations: Map::new(&env),
            confirmation_count: Map::new(&env),
            owners, 
            required_signatures,
            transactions: Map::new(&env),
            next_tx_id: 0,
            pending_additions: Map::new(&env), 
            pending_removals: Map::new(&env),
            owner_modifications_conf: Map::new(&env)

        }; 
        env.storage().instance().set(&MULTISIGSTATE, &new_ms_state);
    }

    pub fn approve_owner_addition(env: Env, owner: Address, caller:Address) {
        assert!(owner != caller); 
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone()));
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        let mut conf_count = ms_state.pending_additions.get(owner.clone()).unwrap_or(0); 
        conf_count += 1; 
        if !ms_state.pending_additions.contains_key(owner.clone()) {
            ms_state.pending_additions.set(owner.clone(), conf_count); 
        }

        if conf_count == ms_state.required_signatures {
            Self::add_owner(env.clone(), owner.clone()); 
            ms_state.pending_additions.remove(owner.clone()); 
        }

        env.storage().instance().set(&MULTISIGSTATE, &ms_state); 
    }

    pub fn approve_owner_removal(env: Env, owner: Address, caller:Address) {
        assert!(owner != caller); 
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone()));
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        let mut conf_count = ms_state.pending_removals.get(owner.clone()).unwrap_or(0); 
        conf_count += 1; 
        if !ms_state.pending_removals.contains_key(owner.clone()) {
            ms_state.pending_removals.set(owner.clone(), conf_count); 
        }

        if conf_count == ms_state.required_signatures {
            Self::remove_owner(env.clone(), owner.clone()); 
            ms_state.pending_removals.remove(owner.clone()); 
        }

        env.storage().instance().set(&MULTISIGSTATE, &ms_state); 
    }

    fn add_owner(env: Env, new_owner: Address) {
        let mut ms_state = Self::get_multisig_state(env.clone());
        ms_state.owners.push_back(new_owner);
        env.storage().instance().set(&MULTISIGSTATE, &ms_state); 
    }

    fn remove_owner(env: Env, owner: Address) {
        let mut ms_state = Self::get_multisig_state(env.clone());
        let index = ms_state.owners.iter().position(|x| x == owner).unwrap() as u32; 
        ms_state.owners.remove(index);
        env.storage().instance().set(&MULTISIGSTATE, &ms_state);
    }

    pub fn submit_tx(env: Env, token: Address, to: Address, amount: i128, caller: Address)
    {
        caller.require_auth(); 
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        assert!(ms_state.owners.contains(caller)); 
        let tx_id = ms_state.next_tx_id; 
        ms_state.next_tx_id += 1;
        let new_tx = ProposedTx {
            token, 
            tx_id,
            transfer_to: to, 
            transfer_amount: amount
        }; 
        ms_state.transactions.set(tx_id, new_tx); 
        env.storage().instance().set(&MULTISIGSTATE, &ms_state);
    }

    pub fn confirm_transaction(env: Env, tx_id: TransactionId, owner: Address) {
        owner.require_auth(); 
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        assert!(ms_state.owners.contains(owner.clone()));
        let mut conf_count = ms_state.confirmation_count.get(tx_id).unwrap_or(0); 
        if !ms_state.confirmations.contains_key((tx_id, owner.clone())) {
            conf_count += 1; 
            ms_state.confirmations.set((tx_id, owner), ()); 
            ms_state.confirmation_count.set(tx_id, conf_count); 
   
        }
        env.storage().instance().set(&MULTISIGSTATE, &ms_state);
    }

    pub fn execute_transaction(env: Env, tx_id: TransactionId) {
        let ms_state = Self::get_multisig_state(env.clone()); 
        assert!(ms_state.confirmation_count.get(tx_id).unwrap() >= ms_state.required_signatures); 
        let tx: ProposedTx = ms_state.transactions.get(tx_id).unwrap(); 

        let token_client = token::Client::new(&env, &tx.token); 
        token_client.transfer(&env.current_contract_address(), &tx.transfer_to, &tx.transfer_amount); 
        
    }

    pub fn is_owner(env: Env, owner: Address) -> bool {
        let ms_state = Self::get_multisig_state(env); 
        ms_state.owners.contains(owner)
    }

    pub fn get_multisig_state(env: Env) -> MultisigState {
        env.storage().instance().get(&MULTISIGSTATE).unwrap() 
    }
   

}

#[cfg(test)]
mod test; 
