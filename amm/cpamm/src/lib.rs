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

pub fn curve_fn(balance_a: i128, balance_b: i128, input: i128) -> Result<i128, SwapCurveError>{
    let divisor = balance_a.checked_add(input).ok_or(SwapCurveError::IntegerOverflow)?;
    let delta = util::rational::safe_mul(balance_a, balance_b, divisor)
        .map_err(|_| SwapCurveError::IntegerOverflow)?;
    let output = balance_b - delta;
    Ok(if output > balance_b{
        -1
    }else{
        output
    })
}

#[contractimpl]
impl ConstantProductSwapCurveContract {
    pub fn compute_swap(env: Env, caller: Address, token_a: Address, token_b: Address, input: i128) -> Result<i128, SwapCurveError>{
        let client_a = soroban_sdk::token::Client::new(&env, &token_a);
        let client_b = soroban_sdk::token::Client::new(&env, &token_b);
        let balance_a = client_a.balance(&caller);
        let balance_b = client_b.balance(&caller);

        curve_fn(balance_a, balance_b, input)
    }
}

#[cfg(test)]
mod test;
