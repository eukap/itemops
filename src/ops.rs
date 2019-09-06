//! Traits for the item operations and their implementations.

use std::default::Default;
use std::ops::{AddAssign, MulAssign};
use std::marker::Copy;

/// An item summing trait.
pub trait ItemSum {
    type Item;
    /// Method for summing items with a specified step. 
    fn sum_step(self, step: usize) -> Self::Item;
}

impl<T> ItemSum for &[T]
    where T: Default + AddAssign + Copy
{
    type Item = T;
    /// Sums elements of a slice with a specified step.  
    /// 
    /// If a slice is empty or a step is equal to zero,
    /// returns the default value of the item type.  
    /// 
    /// # Panics  
    /// 
    /// This method will panic if the computation overflows
    /// and debug assertions are enabled.  
    /// 
    /// # Examples  
    /// 
    /// ```
    /// # use itemops::ops::*;
    /// let v = vec![3, 7, 4, 2, 8, 5]; 
    ///  
    /// assert_eq!(v.sum_step(2), 15);  
    /// assert_eq!(v.sum_step(3), 5);
    /// ```
    fn sum_step(self, step: usize) -> Self::Item {
        let mut res: T = Default::default();
        if step == 0 { return res; }
        let length = self.len();
        let mut idx: usize;
        for i in 0..length {
            idx = i * step;
            if idx < length { res += self[idx]; }
            else { break; }
        }
        res
    }
}

/// An item multiplying trait.
pub trait ItemProduct {
    type Item;
    /// Method for multiplying items with a specified step.
    fn product_step(self, step: usize) -> Self::Item;
}

impl<T> ItemProduct for &[T]
    where T: Default + MulAssign + Copy
{
    type Item = T;
    /// Multiplies elements of a slice with a specified step.  
    /// 
    /// If a slice is empty or a step is equal to zero,
    /// returns the default value of the item type.  
    /// 
    /// # Panics  
    /// 
    /// This method will panic if the computation overflows
    /// and debug assertions are enabled. 
    /// 
    /// # Examples  
    /// 
    /// ```
    /// # use itemops::ops::*;
    /// let a = [2, 5, 1, 6, 3, 7];  
    /// 
    /// assert_eq!(a.product_step(2), 6);
    /// assert_eq!(a.product_step(3), 12);
    /// ```
    fn product_step(self, step: usize) -> Self::Item {
        let mut res: T = Default::default();
        let length = self.len();
        if step == 0 || length == 0 { return res; }
        res = self[0];
        if length == 1 { return res; }
        let mut idx: usize;
        for i in 1..length {
            idx = i * step;
            if idx < length { res *= self[idx]; }
            else { break; }
        }
        res
    }
}
