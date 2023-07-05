use core::marker::PhantomData;

use ::serial::{serialtrait::{Serial, SerialError}};

use crate::motortrait::MovementController;


pub mod serial;
pub struct Servo42C<T: Serial, V: MovementController> {
    pub address: u8,
    pub s: T,
    pub kp: u16,
    pub ki: u16,
    pub kd: u16,
    pub acc: u16,
    pub ph: PhantomData<V>
}

impl<T: Serial, V: MovementController> Servo42C<T, V>{
    pub fn new(s: T)->Result<Servo42C<T, V>, SerialError>{
        let t  =Servo42C::<T, V>{
            address: 0xe0,
            s,
            kp: 1616,
            ki: 288,
            kd: 1616,
            acc: 286,
            ph: PhantomData::default()
        };
        
        Ok(t)
    }

}
