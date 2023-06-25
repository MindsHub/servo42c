
use std::{mem, fmt::Error};
#[derive(Debug)]
pub enum SerialError{
    Undefined
}
pub trait Serial{
    
    fn read(&mut self, buf: &mut [u8])->Result<(), SerialError>;
    fn write(&mut self, buf: &[u8])->Result<(), SerialError>;
}
pub trait Sendable<const S: usize, const U1: usize=0, const U2: usize=0> where //TODO ownership, concat, compile time
{   
    fn as_u8(&self)->[u8; S];
    fn from_u8(x: [u8; S])->Self;
}

macro_rules! get_size {
    ($ecur:literal: $icur:ty) => {
        mem::size_of::<$icur>()
    };
    ($ecur:literal : $i:ty, $($e:literal : $it:ty),*) => {
        mem::size_of::<$i>()+get_size!($($e: $it),*)
    };
}
macro_rules! parse_data {
    ($ecur:literal: $icur:ty) => {
        mem::size_of::<$icur>()
    };
    ($ecur:literal : $iu:ty, $($e:literal : $it:ty),*) => {
        let x = $ecur.as_u8();
        buffer[i..].copy_from_slice(&inp[..XS]);
    };
}

macro_rules! send_data {
    ($n:tt, $ecur:literal: $icur:ty) => {
        mem::size_of::<$icur>()
    };
    ($n:tt, $ecur:literal : $iu:ty, $($e:literal : $it:ty),*) => {
        let x = $ecur.as_u8();
        buffer.copy_from_slice(&inp[..XS]);
    };
    ($($e:literal : $it:ty),+) => {{
        let buffer: [u8; get_size!($($e: $it),*)];
        let mut i=0u8;
        
        send_data!(2, $($e: $it),*);
        //buffer[i..].copy_from_slice(&inp[..XS]);
        //{mem::size_of::<$i>()+get_size!($($e: $it),*)}
    }};
}


fn test(){
    vec![];
    let y: u32=0;
    let t = [0u8; get_size!(7: u32, 4: u8)];
    send_data!(7: u32, 4: u8);
}

/*
pub trait Sendable<const SIZE: usize> where //TODO ownership, concat, compile time
Self: Sized
{
    fn as_u8(&self)->[u8; SIZE];
    fn from_u8(x: [u8; SIZE])->Self;
}

macro_rules! impl_sendable {
    ($XSIZE:literal, $YSIZE:literal) => {
        impl<X: Sendable<$XSIZE>, Y: Sendable<$YSIZE>> Sendable<{$XSIZE+$YSIZE}> for (X, Y) where
            {
                
            fn as_u8(&self)->[u8; ZSIZE] {
                
                const YSIZE: usize=$YSIZE;
                let mut bits = [0; 8];
                let x  =self.0.as_u8();
                bits[..{ZSIZE-YSIZE}].copy_from_slice(&x);
                let y =self.1.as_u8();
                bits[..{ZSIZE-YSIZE}].copy_from_slice(&y);

                todo!()
                //self.0.as_u8().concat()
            }

            fn from_u8(inp: [u8; ZSIZE])->Self {
                const YSIZE: usize=$YSIZE;
                let mut x = [0u8; {ZSIZE-YSIZE}];
                x.copy_from_slice(&inp[..]);
                let x =X::from_u8(x);
                let mut y = [0u8; {YSIZE}];
                y.copy_from_slice(&inp[..]);
                let y =Y::from_u8(y);
                (x, y)
            }
        }
        
    };
}
fn test(){
    (1,2).max(other)
}
impl_sendable!(1,2);
impl_sendable!(2,1);
/**/*/ 

impl<X,Y, const XS: usize, const YS: usize> Sendable<{XS+YS}, XS, YS> for (X, Y) 
    where 
        X: Sendable<XS>,
        Y: Sendable<YS>,
        //[(); X::SIZE + Y::SIZE]:
        {

            fn as_u8(&self)->[u8; XS+YS] {
                let mut bits = [0; XS + YS];
                let x = self.0.as_u8();
                bits[..XS].copy_from_slice(&x);
                let y = self.1.as_u8();
                bits[XS..].copy_from_slice(&y);
                bits
            }

            fn from_u8(inp: [u8; XS+YS]) -> Self {
                let mut x = [0u8; XS];
                x.copy_from_slice(&inp[..XS]);
                let x =X::from_u8(x);
                let mut y = [0u8; YS];
                y.copy_from_slice(&inp[XS..]);
                let y =Y::from_u8(y);
                (x, y)
            }
        }
