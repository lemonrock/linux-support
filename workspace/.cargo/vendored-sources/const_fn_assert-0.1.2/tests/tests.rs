#![no_std]

#[macro_use]
extern crate const_fn_assert;

#[test]
fn test_assert() {
    cfn_assert!(true);
    cfn_assert!(true && (true != false));
    cfn_assert!((true && true) != false);
    cfn_assert_eq!(false, false);
    cfn_assert_ne!(true, false);
}

const fn sub_fn(x: u8) -> u8 {
    cfn_assert!(x < 5);
    x + 1
}

const _TEST_ASSERT: u8 = sub_fn(1);

#[test]
fn test_sub_fn_assert() {
    let _ = sub_fn(1);
}

#[test]
#[should_panic]
fn test_sub_fn_assert_fail() {
    let _ = sub_fn(6);
}

#[test]
const fn test_const_assert() {
    const FIVE: usize = 5;

    cfn_assert!(FIVE * 2 == 10);
    cfn_assert!(FIVE > 2);
}

#[test]
fn test_const_fn_assert() {
    const TEST_CONST_ASSERT: () = test_const_assert();
    let _ = TEST_CONST_ASSERT;
}
