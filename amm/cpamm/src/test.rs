#![cfg(test)]
extern crate std;

use super::*;

#[test]
fn simple_test(){
    assert_eq!(curve_fn(i128::MAX - 1_000_000_000, 3_000_000_000, 2_000_000_000), Err(SwapCurveError::IntegerOverflow));
    assert_eq!(curve_fn(i128::MAX / 100, i128::MAX / 200, 2_000_000_000), Err(SwapCurveError::IntegerOverflow));
    assert_eq!(curve_fn(1_000_000_000, 3_000_000_000, 1_500_000), Ok(4_493_261));
    assert_eq!(curve_fn(3_000_000_000, 1_000_000_000, 1_500_000), Ok(499_751));
    assert_eq!(curve_fn(1_000_000_000, 1_000_000_000, 1_500_000), Ok(1_497_754));
}
