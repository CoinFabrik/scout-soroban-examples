#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol}; 

#[contracttype]
pub struct PaymentChannelState{
    sender: Address,            // The creator of the payment channel who wants to send funds. 
    recipient: Address,         // The receiver of the deposited funds. 
    expiration: Option<u64>,    // The timestamp corresponding to the date the channel is no longer valid. 
    withdrawn: i128,            // The amount that has already been withdrawn by the recipient. 
    token: Address,             // The token the deposit is being made in. 
    allowance: i128,            // The maximum amount the recipient is allowed to withdraw. Sender can update this amount but all withdrawals will sum up to this number. 
                                /* Example: 
                                    1. first allowance: 1000 tokens 
                                    2. user withdrawal: 1000 tokens 
                                    3. sender updates allowance to 1700 tokens 
                                    4. user second withdrawal will be of 700 because of previous withdrawal. 
                                    5. user's final balance: 1700 tokens 
                                */

}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PCError {
    ExpirationNotSet = 1, 
}

const PCSTATE: Symbol = symbol_short!("PC_STATE"); 

#[contract]
pub struct PaymentChannel; 

#[contractimpl]
impl PaymentChannel {

    pub fn initialize(env: Env, sender: Address, recipient: Address, token: Address, allowance: i128) -> PaymentChannelState {
        let new_payment_channel = PaymentChannelState {
            sender,
            recipient, 
            expiration: None,
            withdrawn: 0, 
            token,
            allowance
        };

        env.storage().instance().set(&PCSTATE, &new_payment_channel); 
        Self::get_state(env)
    }

    pub fn get_state(env: Env) -> PaymentChannelState {
        env.storage().instance().get(&PCSTATE).unwrap()
    }

    pub fn close(env: Env) {
        let state = Self::get_state(env.clone()); 
        let recipient = Self::get_recipient_address(env.clone());
        recipient.require_auth();
        let token_client = token::Client::new(&env, &Self::get_state(env.clone()).token);

        if state.allowance > state.withdrawn {
            token_client.transfer(&env.current_contract_address(), &recipient, &(&state.allowance - state.withdrawn)); 
        }
        let remaining_balance = token_client.balance(&env.current_contract_address()); 
        if remaining_balance > 0 {
            token_client.transfer(&env.current_contract_address(), &Self::get_sender_address(env), &remaining_balance); 
        }
    }

    pub fn withdraw(env: Env) {
        Self::get_recipient_address(env.clone()).require_auth();
        let mut state = Self::get_state(env.clone());
        assert!(state.allowance > state.withdrawn); 
        let token_client = token::Client::new(&env, &state.token);
        let amount_to_withdraw = state.allowance - state.withdrawn; 
        state.withdrawn += amount_to_withdraw; 
        token_client.transfer(&env.current_contract_address(), &state.recipient, &amount_to_withdraw); 
        env.storage().instance().set(&PCSTATE, &state); 
    }

    pub fn set_expiration(env: Env, timestamp: u64) {
        Self::get_sender_address(env.clone()).require_auth(); 
        let now = env.ledger().timestamp(); 
        assert!(timestamp > now); 
        let mut state = Self::get_state(env.clone()); 
        state.expiration = Some(timestamp); 
        env.storage().instance().set(&PCSTATE, &state); 
    }

    pub fn claim_timeout(env: Env) -> Result<(), PCError> {
        let state = Self::get_state(env.clone()); 
        state.sender.require_auth();
        match state.expiration {
            Some(expiration) => {
                let now = env.ledger().timestamp(); 
                assert!(now > expiration); 
                let token_client = token::Client::new(&env, &state.token);
                let remaining_balance = token_client.balance(&env.current_contract_address()); 
                if remaining_balance > 0 {
                    token_client.transfer(&env.current_contract_address(), &state.sender, &remaining_balance);
                }
                Ok(())
            }, 
            None => Err(PCError::ExpirationNotSet),
        }
    } 

    pub fn modify_allowance(env: Env, allowance: i128) {
        let mut state = Self::get_state(env.clone());
        state.sender.require_auth();
        assert!(allowance > state.allowance); 
        state.allowance = allowance; 
        env.storage().instance().set(&PCSTATE, &state);
    }

    pub fn get_recipient_address(env: Env) -> Address {
        Self::get_state(env).recipient 
    }

    pub fn get_sender_address(env: Env) -> Address {
        Self::get_state(env).sender
    }

    

}



#[cfg(test)]
mod test; 