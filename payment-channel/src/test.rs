#![cfg(test)]
extern crate std;
//use std::println;

use super::*;

use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{token, Address, Env, IntoVal, Val, Vec};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

#[test]
fn test_payment_channel_with_claim_timeout() {
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

    // INITIALIZE PAYMENT CHANNEL
    token_admin_client.mint(&sender, &5_000); // --> Now sender has balance
                                              

    // Setting sequence
    env.ledger().with_mut(|info| {
        info.sequence_number += 100;
    });
    payment_channel.initialize(&sender, &recipient, &token.address, &1_000, &Some(110));
    let mut state = payment_channel.get_state();
    assert_eq!(state.allowance, 1000);
    assert_eq!(state.sender, sender);
    assert_eq!(state.recipient, recipient);
    assert_eq!(state.expiration, Some(110));
    assert_eq!(state.withdrawn, 0);
    assert_eq!(state.token, token.address);

    // DEPOSIT TOKENS  -- For testing purposes we will just mint to the contract
    /*token_admin_client.mint(&payment_channel.address, &5_000); // --> Now sender has balance*/

    // WITHDRAW
    assert_eq!(token.balance(&payment_channel.address), 1_000);
    assert_eq!(token.balance(&recipient), 0);
    payment_channel.withdraw();
    assert_eq!(
        env.auths(),
        std::vec![(
            recipient.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    payment_channel.address.clone(),
                    symbol_short!("withdraw"),
                    Vec::new(&env)
                )),
                sub_invocations: std::vec![]
            },
        )]
    );
    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&recipient), 1000); // recipient will receive as much as the allowance is

    // MODIFY ALLOWANCE
    payment_channel.modify_allowance(&1_700); // now recipient can withdraw up to 1.700  including what he/she has already extracted.
    let mut params: Vec<i128> = Vec::new(&env);
    let mut params_transfer: Vec<Val> = Vec::new(&env);
    params.push_back(1700);
    params_transfer.push_back(payment_channel.address.into_val(&env));
    params_transfer.push_back(sender.into_val(&env));
    params_transfer.push_back(payment_channel.address.into_val(&env));
    params_transfer.push_back(700.into_val(&env));
    state = payment_channel.get_state();

    assert_eq!(state.allowance, 1_700);

    payment_channel.withdraw();

    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&recipient), 1_700);

    // CLAIM_TIMEOUT

    payment_channel.modify_allowance(&2_000);

    // move time forward to when the contract is already expired
    env.ledger().with_mut(|info| {
        info.sequence_number += 120;
    });

    assert_eq!(token.balance(&sender), 3000);
    payment_channel.claim_timeout(); // when sender claims timeout, the remaining funds go back to him/her
    assert_eq!(
        env.auths(),
        std::vec![(
            sender.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    payment_channel.address.clone(),
                    Symbol::new(&env, "claim_timeout"),
                    Vec::new(&env)
                )),
                sub_invocations: std::vec![]
            },
        )]
    );
    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&sender), 3_300);
    assert_eq!(token.balance(&recipient), 1_700);
}

#[test]
fn test_payment_channel_with_recipient_close() {
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
    token_admin_client.mint(&sender, &5_000); // --> Now sender has balance
    token.approve(&sender, &payment_channel.address, &5000, &300);

    // INITIALIZE PAYMENT CHANNEL

    // Setting sequence number
    env.ledger().with_mut(|info| {
        info.sequence_number += 100;
    });

    payment_channel.initialize(&sender, &recipient, &token.address, &500, &None);

    assert_eq!(token.balance(&payment_channel.address), 500);
    assert_eq!(token.balance(&sender), 4_500);
    // WITHDRAW
    payment_channel.withdraw();
    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&recipient), 500);

    // MODIFY ALLOWANCE
    payment_channel.modify_allowance(&1_000); // user has 500 left to withdraw

    // CLOSE PAYMENT CHANNEL

    payment_channel.close();

    assert_eq!(token.balance(&payment_channel.address), 0);
    assert_eq!(token.balance(&sender), 4_000);
    assert_eq!(token.balance(&recipient), 1_000);
}
