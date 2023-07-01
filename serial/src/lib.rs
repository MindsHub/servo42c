#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use serialtrait::{Sendable, MySize};

pub mod serialtrait;
pub mod test;

pub mod standard;

int_impl!(i32);
int_impl!(i16);
int_impl!(u32);
int_impl!(u16);
int_impl!(u8);

impl MySize for (){}
impl Sendable for () {

    fn into_byte(self)->[u8; 0] {
        [0u8; 0]
    }

    fn from_byte(_: [u8; 0])->Self {
    }
}

/*
//type testArray<const S: usize>=[u8; S];
impl<const S: usize> MySize for [u8; S]{}
impl<const S: usize> Sendable for [u8; S]{
    fn as_u8(self)->[u8; Self::SIZE] {
        let mut y: [u8; Self::SIZE] = [0u8; Self::SIZE];
        y.copy_from_slice(&self);
        y
    }

    fn from_u8(x: [u8; Self::SIZE])->Self {
        let mut y: [u8; S] = [0u8; S];
        y.copy_from_slice(&x);
        y
    }
}*/