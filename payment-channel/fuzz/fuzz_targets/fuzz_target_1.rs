#![no_main]

extern crate payment_channel;
use payment_channel::*;

use libfuzzer_sys::fuzz_target;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, arbitrary::{
    arbitrary,
    fuzz_catch_panic,
    Arbitrary,
},};
use soroban_sdk::{Symbol, symbol_short, token, Address, Env, IntoVal, Val, Vec};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

#[derive(Arbitrary, Debug)]
struct Input {
    balance: i128,
    sequence: u32,
    initialize: i128,
    modify_allowance: i128,
}

fuzz_target!(|input: Input| {
    let env = Env::default();
    env.mock_all_auths();

    let payment_channel =
        PaymentChannelClient::new(&env, &env.register_contract(None, PaymentChannel {}));

    // Initialize sender and recipient
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Create a token for testing purposes
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract(token_admin.clone());
    let token = TokenClient::new(&env, &token_contract);
    let token_admin_client = TokenAdminClient::new(&env, &token_contract);
    token_admin_client.mint(&sender, &input.balance); // --> Now sender has balance
    token.approve(&sender, &payment_channel.address, &input.balance, &300);

    // INITIALIZE PAYMENT CHANNEL

    // Setting sequence number
    env.ledger().with_mut(|info| {
        info.sequence_number += input.sequence;
    });

    payment_channel.initialize(&sender, &recipient, &token.address, &input.initialize, &None);

    assert_eq!(token.balance(&payment_channel.address), input.initialize);
    assert_eq!(token.balance(&sender), (input.balance - input.initialize));
    // WITHDRAW
    payment_channel.withdraw();
    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&recipient), input.initialize);

    // MODIFY ALLOWANCE
    payment_channel.modify_allowance(&input.modify_allowance); // user has 500 left to withdraw

    // CLOSE PAYMENT CHANNEL

    payment_channel.close();

    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&sender), (input.balance - input.initialize - input.initialize));
    assert_eq!(token.balance(&recipient), (input.initialize + input.initialize));
});
