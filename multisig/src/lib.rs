#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, Symbol, Vec};


#[derive(Debug, Clone, PartialEq)]
#[contracttype]
pub struct ProposedTx {         // The proposed transaction to be executed
    pub token: Address,         // Token to be transferred
    pub tx_id: u32,             // Transaction ID
    pub transfer_to: Address,   // The receiver of the transfer
    pub transfer_amount: i128,  // The amount that will be sent 
    pub executed: bool          // True if the transaction has already been executed
}

#[contracttype]
#[derive(Debug)]
pub struct MultisigState {
    confirmations: Map<(u32 /*tx_id*/, Address), ()>,                         // Keeps record of what owners have already voted a transaction. 
    confirmation_count: Map<u32 /*tx_id*/, u32>,                              // Keeps record of how many confirmations a transaction has. 
    transactions: Map<u32 /*tx_id*/, ProposedTx>,                             // Keeps record of transactions. 
    owners: Vec<Address>,                                                     // The current owners of the multisig. 
    required_signatures: u32,                                                 // The amount of needed signatures in order to execute a transaction. 
    next_tx_id: u32,                                                          // Next transaction id. 
    pending_modifications: Map<(Address, bool), u32>,                         // Keeps record of owner addition/removal proposals and votes. When key contians `true` it refers to an addition, when `false`, to a removal. Address -> owner to be removed or added / bool -> addition/removal / u32 -> amount of confirmations. 
    owner_modifications_conf: Map<(Address, Address, bool), ()>               // Keeps record of what owners have voted an addition/removal. Address -> owner to be removed/added / Address -> owner who voted / bool -> addition/removal. 
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
            pending_modifications: Map::new(&env), 
            owner_modifications_conf: Map::new(&env)

        }; 
        env.storage().instance().set(&MULTISIGSTATE, &new_ms_state);
    }

    pub fn approve_owner_addition(env: Env, new_owner: Address, caller:Address) {
        
        assert!(new_owner != caller); 
        assert!(!Self::is_owner(env.clone(), new_owner.clone()));
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone()));
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        let mut conf_count = ms_state.pending_modifications.get((new_owner.clone(), true)).unwrap_or(0); 
        conf_count += 1; 

        assert!(!ms_state.owner_modifications_conf.contains_key((new_owner.clone(), caller.clone(), true)));
        ms_state.owner_modifications_conf.set((new_owner.clone(), caller.clone(), true), ());
        
        ms_state.pending_modifications.set((new_owner.clone(),true), conf_count); 
       

        if conf_count == ms_state.required_signatures {
            ms_state = Self::add_owner(env.clone(), new_owner.clone()); 
            ms_state.pending_modifications.remove((new_owner.clone(), true)); 
        }

        env.storage().instance().set(&MULTISIGSTATE, &ms_state); 
    }

    fn add_owner(env: Env, new_owner: Address) -> MultisigState {
        let mut ms_state = Self::get_multisig_state(env.clone());
        ms_state.owners.push_back(new_owner.clone());

        for (key, _) in ms_state.owner_modifications_conf.clone() {
            if key.0 == new_owner.clone() && key.2 == true {
                ms_state.owner_modifications_conf.remove(key);
            }
        }

        ms_state
    }

    

    pub fn approve_owner_removal(env: Env, owner: Address, caller:Address) {
        assert!(owner != caller); 
        assert!(Self::is_owner(env.clone(), owner.clone()));
        caller.require_auth();
        assert!(Self::is_owner(env.clone(), caller.clone()));
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        let mut conf_count = ms_state.pending_modifications.get((owner.clone(), false)).unwrap_or(0); 
        conf_count += 1; 

        assert!(!ms_state.owner_modifications_conf.contains_key((owner.clone(), caller.clone(), false))); //cannot vote twice
        ms_state.owner_modifications_conf.set((owner.clone(), caller.clone(), false), ());
        ms_state.pending_modifications.set((owner.clone(),false), conf_count); 
        

        if conf_count == ms_state.required_signatures {
            ms_state = Self::remove_owner(env.clone(), owner.clone()); 
            ms_state.pending_modifications.remove((owner.clone(), false)); 
        }

        env.storage().instance().set(&MULTISIGSTATE, &ms_state); 
    }

    fn remove_owner(env: Env, owner: Address) -> MultisigState {
        let mut ms_state = Self::get_multisig_state(env.clone());
        let index = ms_state.owners.iter().position(|x| x == owner).unwrap() as u32; 
        ms_state.owners.remove(index);

        for (key, _) in ms_state.owner_modifications_conf.clone() {
            if key.0 == owner.clone() && key.2 == false {
                ms_state.owner_modifications_conf.remove(key);
            }
        }

        ms_state
        
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
            transfer_amount: amount,
            executed: false
        }; 
        ms_state.transactions.set(tx_id, new_tx); 
        env.storage().instance().set(&MULTISIGSTATE, &ms_state);
    }

    pub fn confirm_transaction(env: Env, tx_id: u32, owner: Address) {
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

    pub fn execute_transaction(env: Env, tx_id: u32) {
        let mut ms_state = Self::get_multisig_state(env.clone()); 
        assert!(ms_state.confirmation_count.get(tx_id).unwrap() >= ms_state.required_signatures); 
        let mut tx: ProposedTx = ms_state.transactions.get(tx_id).unwrap(); 
        assert!(!tx.executed);

        let token_client = token::Client::new(&env, &tx.token); 
        token_client.transfer(&env.current_contract_address(), &tx.transfer_to, &tx.transfer_amount); 
        tx.executed = true; 

        ms_state.transactions.set(tx_id, tx); 
        env.storage().instance().set(&MULTISIGSTATE, &ms_state);
        
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


