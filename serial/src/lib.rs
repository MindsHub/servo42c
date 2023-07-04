#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![cfg_attr(not(feature = "std"), no_std)]
//#![no_std]

use serialtrait::{MySize, Sendable};

pub mod serialtrait;


#[cfg(feature ="std")]
pub mod standard;
#[cfg(feature ="std")]
pub mod test;

int_impl!(i32);
int_impl!(i16);
int_impl!(u32);
int_impl!(u16);
int_impl!(u8);

impl MySize for () {}
impl Sendable for () {
    fn into_byte(self) -> [u8; 0] {
        [0u8; 0]
    }

    fn from_byte(_: [u8; 0]) -> Self {}
}
