#![allow(dead_code)]

use allot_codegen::lib_return;

type Return = (
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<i32>,
);

#[test]
fn t1() {
    assert_eq!((Some(1), Some(2), Some(3), Some(4), Some(5)), t1_ret())
}

fn t1_ret() -> Return {
    lib_return!(1, 2, 3, 4, 5);
}

#[test]
fn t2() {
    assert_eq!((Some(1), None, None, None, None), t2_ret())
}

fn t2_ret() -> Return {
    lib_return!(1)
}

#[test]
fn t3() {
    assert_eq!((None, None, None, None, None), t3_ret())
}

fn t3_ret() -> Return {
    lib_return!();
}
