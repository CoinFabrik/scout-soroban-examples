#![no_std]

use soroban_sdk::{
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

pub mod rational;

pub struct SwapCurveInterfaceSpec;

#[contractspecfn(name = "SwapCurveInterfaceSpec", export = false)]
#[contractclient(crate_path = "crate", name = "SwapCurveClient")]
pub trait SwapCurveInterface {
    fn compute_swap(env: Env, caller: Address, token_a: Address, token_b: Address, input: i128) -> i128;
}
