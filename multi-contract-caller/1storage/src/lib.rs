#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Env,
};

#[contract]
pub struct StorageContract;

#[contracttype]
pub enum DataKey {
    Data,
}

#[contractimpl]
impl StorageContract {
    pub fn get(env: Env) -> i64{
        env.storage().instance().get(&DataKey::Data).or(Some(0_i64)).unwrap()
    }
    pub fn set(env: Env, value: i64){
        env.storage().instance().set(&DataKey::Data, &value);
    }
}

#[cfg(test)]
mod test;
