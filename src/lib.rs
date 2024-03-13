#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, token, xdr::ToXdr, Address, BytesN, Env, Symbol}; 

#[contracttype]
pub struct PaymentChannelState{
    sender: Address,
    recipient: Address, 
    expiration: Option<i128>, 
    withdrawn: i128, 
    close_duration: i128,
    token: Address
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

    pub fn initialize(env: Env, sender: Address, recipient: Address, close_duration: i128, token: Address) -> PaymentChannelState {
        let new_payment_channel = PaymentChannelState {
            sender,
            recipient, 
            expiration: None,
            withdrawn: 0, 
            close_duration,
            token
        };

        env.storage().instance().set(&PCSTATE, &new_payment_channel); 
        Self::get_state(env)
    }

    pub fn get_state(env: Env) -> PaymentChannelState {
        env.storage().instance().get(&PCSTATE).unwrap()
    }

    pub fn close(env: Env, amount: i128, signature: BytesN<64>) {
        let mut state = Self::get_state(env.clone());

        let sender = Self::get_sender_address(env.clone());
        verify_signature(env.clone(), amount, signature, sender);
        let recipient = Self::get_recipient_address(env.clone());
        recipient.require_auth();
        let token_client = token::Client::new(&env, &Self::get_state(env.clone()).token);
        
        assert!(amount > state.withdrawn); 

        token_client.transfer(&env.current_contract_address(), &recipient, &(amount - state.withdrawn)); 
        let remaining_balance = token_client.balance(&env.current_contract_address()); 
        if remaining_balance > 0 {
            token_client.transfer(&env.current_contract_address(), &Self::get_sender_address(env), &remaining_balance); 
        }
    }

    pub fn withdraw(env: Env, amount: i128, signature: BytesN<64>) {
        Self::get_recipient_address(env.clone()).require_auth();

        let sender = Self::get_sender_address(env.clone());
        verify_signature(env.clone(), amount, signature, sender);

        let mut state = Self::get_state(env.clone());
        let token_client = token::Client::new(&env, &state.token);

        assert!(amount > state.withdrawn); 

        let amount_to_withdraw = amount - state.withdrawn; 
        state.withdrawn += amount_to_withdraw; 
        token_client.transfer(&env.current_contract_address(), &state.recipient, &amount_to_withdraw); 
        env.storage().instance().set(&PCSTATE, &state); 
    }

    pub fn set_expiration(env: Env, timestamp: i128) {
        Self::get_sender_address(env.clone()).require_auth(); 
        let now = env.ledger().timestamp() as i128; 
        assert!(timestamp > now); 
        let mut state = Self::get_state(env.clone()); 
        state.expiration = Some(timestamp); 
        env.storage().instance().set(&PCSTATE, &state); 
    }

    pub fn claim_timeout(env: Env) -> Result<(), PCError> {
        let state = Self::get_state(env.clone()); 
        match state.expiration {
            Some(expiration) => {
                let now = env.ledger().timestamp() as i128; 
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

    pub fn get_recipient_address(env: Env) -> Address {
        Self::get_state(env).recipient 
    }

    pub fn get_sender_address(env: Env) -> Address {
        Self::get_state(env).sender
    }

    

}

fn verify_signature(env: Env, amount: i128, signature: BytesN<64>, sender_pubkey: Address) {
    let contract_id_bytes = env.current_contract_address().to_xdr(&env); 
    let amount_to_bytes = amount.to_xdr(&env);
    contract_id_bytes.append(&amount_to_bytes);

    let encodable_hash = env.crypto().sha256(&contract_id_bytes); 
    let state = symbol_short!("PC_STATE");
    // will panic if signature verification fails 
    env.crypto().ed25519_verify(&BytesN::try_from(sender_pubkey.to_xdr(&env)), &encodable_hash, &signature); 

}



#[cfg(test)]
mod test; 

