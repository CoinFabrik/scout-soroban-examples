#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{symbol_short, token, Address, Env, IntoVal};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_claimable_balance_contract<'a>(e: &Env) -> VestingContractClient<'a> {
    VestingContractClient::new(e, &e.register_contract(None, VestingContract {}))
}

struct VestingTest<'a> {
    env: Env,
    token: TokenClient<'a>,
    token_admin: TokenAdminClient<'a>,
    admin: Address,
    beneficiary: Address,
    contract: VestingContractClient<'a>,
}

impl<'a> VestingTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|info| {
            info.timestamp = 12345;
        });

        let admin = Address::generate(&env);
        let beneficiary = Address::generate(&env);

        let token_admin = Address::generate(&env);

        let contract_address = env.register_stellar_asset_contract(token_admin.clone());
        let token = TokenClient::new(&env, &contract_address);
        let token_admin_client = TokenAdminClient::new(&env, &contract_address);
        token_admin_client.mint(&admin, &1_000_000_000_000);

        let contract = create_claimable_balance_contract(&env);
        VestingTest {
            env,
            token,
            token_admin: token_admin_client,
            admin,
            beneficiary,
            contract,
        }
    }
}

#[test]
fn test_creation(){
    let vesting = VestingTest::setup();

    let id = vesting.contract.new_vesting(
        &vesting.token.address,
        &vesting.beneficiary,
        &(vesting.env.ledger().timestamp() + 86400),
        &86400,
        &vesting.admin
    );

    assert_eq!(id, 0);
    assert_eq!(vesting.contract.retrievable_balance(&id), 0);

    let total_vested = vesting.contract.add_vest(&id, &vesting.token.address, &vesting.admin, &1_000_000_000);

    let env = &vesting.env;

    // Verify authorizations
    assert_eq!(
        env.auths(),
        std::vec![
            (
                vesting.admin.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        vesting.contract.address.clone(),
                        symbol_short!("add_vest"),
                        (id.clone(), vesting.token.address.clone(), vesting.admin.clone(), 1_000_000_000_i128).into_val(env),
                    )),
                    sub_invocations: std::vec![]
                },
            ),
            (
                vesting.admin.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        vesting.contract.address.clone(),
                        symbol_short!("add_vest"),
                        (id.clone(), vesting.token.address.clone(), vesting.admin.clone(), 1_000_000_000_i128).into_val(env),
                    )),
                    sub_invocations: std::vec![
                        AuthorizedInvocation {
                            function: AuthorizedFunction::Contract((
                                vesting.token.address.clone(),
                                symbol_short!("transfer"),
                                (vesting.admin.clone(), vesting.contract.address.clone(), 1_000_000_000_i128).into_val(env),
                            )),
                            sub_invocations: std::vec![]
                        }
                    ]
                }
            ),
        ]
    );

    assert_eq!(total_vested, 1_000_000_000);
    assert_eq!(vesting.contract.retrievable_balance(&id), 0);

    vesting.env.ledger().with_mut(|info|{
        info.timestamp += 86399;
    });

    assert_eq!(vesting.contract.retrievable_balance(&id), 0);

    vesting.env.ledger().with_mut(|info|{
        info.timestamp += 2;
    });

    assert_eq!(vesting.contract.retrievable_balance(&id), 1_000_000_000 / 86400);

    vesting.env.ledger().with_mut(|info|{
        info.timestamp += 86400 / 2 - 1;
    });

    assert_eq!(vesting.contract.retrievable_balance(&id), 500_000_000);

    let paid_out = vesting.contract.pay_out(&id);

    assert_eq!(paid_out, 500_000_000);
    assert_eq!(vesting.contract.retrievable_balance(&id), 0);

    let total_vested = vesting.contract.add_vest(&id, &vesting.token.address, &vesting.admin, &1_500_000_000);

    assert_eq!(total_vested, 2_500_000_000);
    assert_eq!(vesting.contract.retrievable_balance(&id), 2_500_000_000 / 2 - paid_out);

    vesting.env.ledger().with_mut(|info|{
        info.timestamp += 86400;
    });

    assert_eq!(vesting.contract.retrievable_balance(&id), 2_000_000_000);

}
