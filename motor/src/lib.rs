//#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![cfg_attr(not(feature = "std"), no_std)]
pub mod motortrait;
pub mod servo42;
pub mod test;
pub use serial;
