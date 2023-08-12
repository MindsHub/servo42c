use core::mem;
#[derive(Debug)]
pub enum SerialError {
    Undefined,
    ConnectionBreak,
}

pub trait Serial {
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SerialError>;
    fn write(&mut self, buf: &[u8]) -> Result<(), SerialError>;
}

pub trait MySize
where
    Self: Sized,
{
    const SIZE: usize = mem::size_of::<Self>();
}
pub trait Sendable
where
    Self: MySize,
{
    fn into_byte(self) -> [u8; Self::SIZE];
    fn from_byte(x: [u8; Self::SIZE]) -> Self;
}

impl<X: MySize, Y: MySize> MySize for (X, Y) {
    const SIZE: usize = X::SIZE + Y::SIZE;
}
impl<X, Y> Sendable for (X, Y)
where
    X: Sendable,
    Y: Sendable,
    [(); X::SIZE]:,
    [(); Y::SIZE]:,
{
    fn into_byte(self) -> [u8; Self::SIZE] {
        let mut bits = [0; Self::SIZE];
        let x = self.0.into_byte();
        bits[..X::SIZE].copy_from_slice(&x);
        let y = self.1.into_byte();
        bits[X::SIZE..].copy_from_slice(&y);
        bits
    }

    fn from_byte(inp: [u8; Self::SIZE]) -> Self {
        let mut x = [0u8; X::SIZE];
        x.copy_from_slice(&inp[..X::SIZE]);
        let x = X::from_byte(x);
        let mut y = [0u8; Y::SIZE];
        y.copy_from_slice(&inp[X::SIZE..]);
        let y = Y::from_byte(y);
        (x, y)
    }
}

#[macro_export]
macro_rules! int_impl {
    ($arg:ty) => {
        impl MySize for $arg {}
        impl Sendable for $arg {
            fn into_byte(self) -> [u8; <$arg>::SIZE] {
                self.to_be_bytes()
            }
            fn from_byte(x: [u8; <$arg>::SIZE]) -> Self {
                Self::from_be_bytes(x.try_into().unwrap())
            }
        }
    };
}
