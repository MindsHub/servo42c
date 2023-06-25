#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

//use serialtrait::Sendable;

use serialtrait::Sendable;

pub mod serialtrait;
pub mod test;
pub mod standard;

impl Sendable<4> for u32 {
    
    fn as_u8(&self)->[u8; 4] {
        self.to_le_bytes()
    }

    fn from_u8(x: [u8; 4])->Self {
        Self::from_le_bytes(x.try_into().unwrap())
    }

    //const SIZE: usize = 1;
}


impl Sendable<1> for u8 {

    fn as_u8(&self)->[u8; 1] {
        [*self; 1]
    }

    fn from_u8(x: [u8; 1])->Self {
        x[0]
    }
}
impl<const S: usize> Sendable<S> for [u8; S]{
    fn as_u8(&self)->[u8; S] {
        self.clone()
    }

    fn from_u8(x: [u8; S])->Self {
        x
    }
}

