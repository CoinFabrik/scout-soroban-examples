#![no_std]

use soroban_sdk::{
    contractspecfn,
    contractclient,
    contracterror,
    Env,
    Address,
    vec,
    Val,
    Symbol,
    FromVal,
    TryFromVal,
    IntoVal,
    InvokeError,
};
#[cfg(any(test, feature = "testutils"))]
use soroban_sdk::{
    testutils,
    xdr,
};

pub mod rational;

pub struct SwapCurveInterfaceSpec;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SwapCurveError {
    IntegerOverflow = 1,
    CannotFulfillSwap = 2,
}

#[contractspecfn(name = "SwapCurveInterfaceSpec", export = false)]
#[contractclient(crate_path = "crate", name = "SwapCurveClient")]
pub trait SwapCurveInterface {
    fn compute_swap(env: Env, caller: Address, token_a: Address, token_b: Address, input: i128) -> Result<i128, SwapCurveError>;
}
