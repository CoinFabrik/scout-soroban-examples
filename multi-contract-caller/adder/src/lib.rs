#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contractspecfn,
    contractclient,
    Env,
    Address,
    vec,
    Val,
    Symbol,
    FromVal,
    TryFromVal,
    IntoVal,
    Error,
    InvokeError,
};
#[cfg(any(test, feature = "testutils"))]
use soroban_sdk::{
    testutils,
    xdr,
};

#[contract]
pub struct AdderContract;

struct StorageInterfaceSpec;

#[contractspecfn(name = "StorageInterfaceSpec", export = false)]
#[contractclient(crate_path = "crate", name = "StorageClient")]
pub trait StorageInterface {
    fn get(env: Env) -> i64;
    fn set(env: Env, value: i64) -> ();
}

#[contractimpl]
impl AdderContract {
    pub fn do_it(env: Env, storage: Address, x: i64) -> i64{
        let c = StorageClient::new(&env, &storage);
        let value = c.get();
        let value = value.saturating_add(x);
        c.set(&value);
        value
    }
}

#[cfg(test)]
mod test;
