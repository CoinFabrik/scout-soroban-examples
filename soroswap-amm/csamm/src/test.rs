#![cfg(test)]
extern crate std;

use super::*;

#[test]
fn simple_test(){
    assert_eq!(curve_fn(1_000_000_000, 3_000_000_000, 1_000).unwrap(), 1_000);
    assert_eq!(curve_fn(3_000_000_000, 1_000_000_000, 2_000_000_000).unwrap(), 1_000_000_000);
}
