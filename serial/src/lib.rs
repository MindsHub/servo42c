#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use serialtrait::{MySize, Sendable};

pub mod serialtrait;
pub mod test;

pub mod standard;

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
