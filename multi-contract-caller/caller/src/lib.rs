#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    contracterror,
    contractspecfn,
    contractclient,
    vec,
    Val,
    Symbol,
    FromVal,
    TryFromVal,
    IntoVal,
    Error,
    InvokeError,
    Env,
    Address,
};
#[cfg(any(test, feature = "testutils"))]
use soroban_sdk::{
    testutils,
    xdr,
};

#[contract]
pub struct CallerContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub storage: Address,
    pub adder: Address,
    pub subber: Address,
    pub which: bool,
}

impl State{
    pub fn new(storage: Address, adder: Address, subber: Address) -> State{
        State{
            storage,
            adder,
            subber,
            which: false,
        }
    }
}

#[contracttype]
pub enum DataKey {
    State,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CallerError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
}

pub struct DoerInterfaceSpec;

#[contractspecfn(name = "DoerInterfaceSpec", export = false)]
#[contractclient(crate_path = "crate", name = "DoerClient")]
pub trait DoerInterface {
    fn do_it(env: Env, storage: Address, x: i64) -> i64;
}

#[contractimpl]
impl CallerContract {
    pub fn init(env: Env, storage: Address, adder: Address, subber: Address) -> Result<(), CallerError>{
        if !env.storage().instance().get::<DataKey, State>(&DataKey::State).is_none() {
            return Err(CallerError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::State, &State::new(storage, adder, subber));
        Ok(())
    }
    pub fn flip(env: Env) -> Result<(), CallerError>{
        let mut state: State = env.storage().instance().get(&DataKey::State).ok_or(CallerError::NotInitialized)?;
        state.which = !state.which;
        env.storage().instance().set(&DataKey::State, &state);
        Ok(())
    }
    pub fn variable_do_it(env: Env, x: i64) -> Result<i64, CallerError>{
        let state: State = env.storage().instance().get(&DataKey::State).ok_or(CallerError::NotInitialized)?;
        let addr = if !state.which{
            state.adder
        }else{
            state.subber
        };
        Ok(DoerClient::new(&env, &addr).do_it(&state.storage, &x))
    }
}

#[cfg(test)]
mod test;

