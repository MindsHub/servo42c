use ::serial::{serialtrait::{Serial, SerialError}};


pub mod serial;
pub struct Servo42C<T: Serial> {
    pub address: u8,
    pub s: T,
    pub kp: u16,
    pub ki: u16,
    pub kd: u16,
    pub acc: u16,
}

impl<T: Serial> Servo42C<T>{
    pub fn new(s: T)->Result<Servo42C<T>, SerialError>{
        let t  =Servo42C::<T>{
            address: 0xe0,
            s,
            kp: 1616,
            ki: 288,
            kd: 1616,
            acc: 286,
        };
        
        Ok(t)
    }

}
