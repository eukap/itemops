# ![Crates.io](https://img.shields.io/crates/v/itemops?style=plastic) ![Crates.io](https://img.shields.io/crates/l/itemops?style=plastic)

## Description

itemops provides some operations on items of Rust language slices.

## Examples

For summing elements of a slice with a specified step:

```rust
let v = vec![3, 7, 4, 2, 8, 5];

assert_eq!(v.sum_step(2), 15);
assert_eq!(v.sum_step(3), 5);
```

For multiplying elements of a slice with a specified step:

```rust
let a = [2, 5, 1, 6, 3, 7];

assert_eq!(a.product_step(2), 6);
assert_eq!(a.product_step(3), 12);
```
