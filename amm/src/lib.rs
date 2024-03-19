#![no_std]

use soroban_sdk::{
    contract,
    contracterror,
    contractimpl,
    contracttype,
    BytesN,
    Address,
    Env,
    IntoVal,
};
use num_integer::Roots;
use util::rational::Ratio;

#[contract]
pub struct SwapContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalState {
    pub admin: Address,
    pub token_wasm_hash: BytesN<32>,
}

impl GlobalState{
    pub fn new(admin: Address, token_wasm_hash: BytesN<32>) -> GlobalState{
        GlobalState{
            admin,
            token_wasm_hash,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub token_a: Address,
    pub token_b: Address,
    pub liq_token: Address,
    pub swap_curve: Address,
}

impl State{
    pub fn new(token_a: Address, token_b: Address, liq_token: Address, swap_curve: Address) -> State{
        State{
            token_a,
            token_b,
            liq_token,
            swap_curve,
        }
    }
}

#[contracttype]
pub enum DataKey {
    GlobalState,
    State(Address, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SwapError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    SameTokens = 3,
    SwapAlreadyInitialized = 4,
    SwapNotInitialized = 5,
    IntegerOverflow = 6,
    ZeroDeposit = 7,
    DepositRejected = 8,
    CannotFulfillSwap = 9,
    SwapRejected = 10,
    InvalidAdmin = 11,
}

mod token;

#[contractimpl]
impl SwapContract {
    pub fn initialize(env: Env, admin: Address, token_wasm_hash: BytesN<32>) -> Result<(), SwapError>{
        if !env.storage().instance().get::<DataKey, GlobalState>(&DataKey::GlobalState).is_none() {
            return Err(SwapError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::GlobalState, &GlobalState::new(admin, token_wasm_hash));
        Ok(())
    }

    fn sort_tokens(token_a: Address, token_b: Address) -> (Address, Address){
        if token_a < token_b{
            (token_a, token_b)
        }else{
            (token_b, token_a)
        }
    }

    fn sort_token_deposits(token_a: (Address, i128, i128), token_b: (Address, i128, i128)) -> ((Address, i128, i128), (Address, i128, i128)){
        if token_a.0 < token_b.0{
            (token_a, token_b)
        }else{
            (token_b, token_a)
        }
    }

    pub fn new_swap(env: Env, admin: Address, token_a: Address, token_b: Address, swap_curve: Address) -> Result<(), SwapError>{
        if token_a == token_b{
            return Err(SwapError::SameTokens);
        }

        let global_state: GlobalState = env.storage().instance()
            .get(&DataKey::GlobalState)
            .ok_or(SwapError::NotInitialized)?;

        if admin != global_state.admin{
            return Err(SwapError::InvalidAdmin);
        }

        admin.require_auth();

        let (token_a, token_b) = SwapContract::sort_tokens(token_a, token_b);
        let key = DataKey::State(token_a.clone(), token_b.clone());

        if !env.storage().instance().get::<DataKey, State>(&key).is_none(){
            return Err(SwapError::SwapAlreadyInitialized);
        }

        let contract = token::create_contract(&env, global_state.token_wasm_hash, &token_a, &token_b);
        let client = token::Client::new(&env, &contract);
        client.initialize(
            &env.current_contract_address(),
            &7u32,
            &"Pool Share Token".into_val(&env),
            &"POOL".into_val(&env),
        );

        env.storage().instance().set(&key, &State::new(token_a, token_b, client.address, swap_curve));

        Ok(())
    }

    pub fn deposit(env: Env, from: Address, token_a: (Address, i128, i128), token_b: (Address, i128, i128), recipient: Address) -> Result<i128, SwapError>{
        let global_state: GlobalState = env.storage().instance()
            .get(&DataKey::GlobalState)
            .ok_or(SwapError::NotInitialized)?;
        let is_admin = from == global_state.admin;

        from.require_auth();

        let (token_a, token_b) = SwapContract::sort_token_deposits(token_a, token_b);

        let state: State = env.storage().instance()
            .get(&DataKey::State(token_a.0.clone(), token_b.0.clone()))
            .ok_or(SwapError::SwapNotInitialized)?;

        let client_a = soroban_sdk::token::Client::new(&env, &token_a.0);
        let client_b = soroban_sdk::token::Client::new(&env, &token_b.0);

        let mut amount_a = token_a.1.min(client_a.balance(&from));
        let mut amount_b = token_b.1.min(client_b.balance(&from));

        let me = env.current_contract_address();

        if !is_admin{
            let desired = Ratio::new(client_a.balance(&me), client_b.balance(&me));
            let actual = Ratio::new(amount_a, amount_b);

            let cmp = actual.compare(&desired);
            if cmp > 0{
                amount_a = desired.truncating_mul(amount_b);
            }else if cmp < 0{
                amount_b = Ratio::new(amount_a, 1).mul(&desired.reciprocal()).to_i128();
            }

            if amount_a < token_a.2 || amount_b < token_b.2{
                return Err(SwapError::DepositRejected);
            }
        }

        let liquidity = amount_a
            .checked_mul(amount_b)
            .ok_or(SwapError::AlreadyInitialized)?;
        if liquidity == 0{
            return Err(SwapError::ZeroDeposit);
        }
        let liquidity = liquidity.sqrt();

        client_a.transfer(&from, &me, &amount_a);
        client_b.transfer(&from, &me, &amount_b);

        let admin_client = token::Client::new(&env, &state.liq_token);
        admin_client.mint(&recipient, &liquidity);
        
        Ok(liquidity)
    }

    pub fn swap(env: Env, from: Address, to: Address, token_a: Address, token_b: Address, input: i128, min_output: i128) -> Result<i128, SwapError>{
        let client_a = soroban_sdk::token::Client::new(&env, &token_a);
        let client_b = soroban_sdk::token::Client::new(&env, &token_b);

        let input = input.min(client_a.balance(&from));
        if input < 1{
            return Ok(0);
        }

        let (sorted_a, sorted_b) = SwapContract::sort_tokens(token_a.clone(), token_b.clone());

        let state: State = env.storage().instance()
            .get(&DataKey::State(sorted_a, sorted_b))
            .ok_or(SwapError::SwapNotInitialized)?;

        let curve = util::SwapCurveClient::new(&env, &state.swap_curve);

        let output = curve.compute_swap(&env.current_contract_address(), &token_a, &token_b, &input);

        if output < 0{
            return Err(SwapError::CannotFulfillSwap);
        }
        if output < min_output{
            return Err(SwapError::SwapRejected);
        }

        let me = env.current_contract_address();

        from.require_auth();
        client_a.transfer(&from, &me, &input);
        client_b.transfer(&me, &to, &output);

        Ok(output)
    }
}

#[cfg(test)]
mod test;
