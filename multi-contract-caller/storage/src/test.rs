#![cfg(test)]
extern crate std;

use super::*;

fn create_contract<'a>(e: &Env) -> StorageContractClient<'a> {
    StorageContractClient::new(e, &e.register_contract(None, StorageContract {}))
}

#[test]
fn test_creation(){
    let env = Env::default();
    env.mock_all_auths();

    let storage = create_contract(&env);

    assert_eq!(storage.get(), 0);
    storage.set(&42);
    assert_eq!(storage.get(), 42);
    storage.set(&0x7FFFFFFF_FFFFFFFF_i64);
    assert_eq!(storage.get(), 0x7FFFFFFF_FFFFFFFF_i64);
}

