extern crate itemops;

use itemops::ops::*;

#[test]
fn sum_step_works() {
    let mut v = vec![5, 6, 14, 2, 17, 21];
    assert_eq!(v.sum_step(2), 36);
    assert_eq!(v[1..4].sum_step(1), 22);
    assert_eq!(v.sum_step(3), 7);

    assert_eq!(v.sum_step(0), 0);
    assert_eq!(v.sum_step(6), 5);
    v.clear();
    assert_eq!(v.sum_step(2), 0);
}

#[test]
#[should_panic]
fn sum_step_panics() {
    let a = [std::i32::MAX, 1];
    a.sum_step(1);
}

#[test]
fn product_step_works() {
    let a = [7, 2, 9, 4, 5, 3];
    assert_eq!(a.product_step(1), 7560);
    assert_eq!(a.product_step(2), 315);
    assert_eq!(a.product_step(3), 28);
    assert_eq!(a.product_step(0), 0);
    assert_eq!(a.product_step(6), 7);
}

#[test]
#[should_panic]
fn product_step_panics() {
    let v = vec![std::i16::MAX, 2];
    v.product_step(1);
}
