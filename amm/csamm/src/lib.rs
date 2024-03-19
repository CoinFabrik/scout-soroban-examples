#![no_std]
use soroban_sdk::{
    contract,
    contractimpl,
    contracterror,
    Address,
    Env,
};

#[contract]
pub struct ConstantProductSwapCurveContract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SwapCurveError {
    IntegerOverflow = 1,
}

pub fn curve_fn(_balance_a: i128, balance_b: i128, input: i128) -> Result<i128, SwapCurveError>{
    Ok(balance_b.min(input))
}

#[contractimpl]
impl ConstantProductSwapCurveContract {
    pub fn compute_swap(env: Env, caller: Address, _token_a: Address, token_b: Address, input: i128) -> Result<i128, SwapCurveError>{
        //let client_a = soroban_sdk::token::Client::new(&env, &token_a);
        let client_b = soroban_sdk::token::Client::new(&env, &token_b);
        //let balance_a = client_a.balance(&caller);
        let balance_b = client_b.balance(&caller);

        curve_fn(/*balance_a*/0, balance_b, input)
    }
}

#[cfg(test)]
mod test;
