#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use serial::serialtrait::{Serial, Sendable, SerialError};
use serial::serialtrait::MySize;
pub struct Servo42C<T: Serial>{
    address: u8,
    s: T,
}

impl<T: Serial> Servo42C<T>{
    pub fn send_cmd<Data: Sendable, Res: Sendable>(&mut self, code: u8, data: Data)->Result<Res, SerialError>
        where 
        [(); <((u8, u8), Data)>::SIZE]:,
        [(); Data::SIZE]:,
        [(); Data::SIZE+3]:,
        [(); Res::SIZE+2]:,
        [(); <(u8, Res)>::SIZE]:,
        [(); <((u8, Res), u8)>::SIZE]:,
        {
        let data_buf =((self.address, code), data).as_u8() ;
        let chksm: u8 = (data_buf.iter().fold(0u32, |acc, x|acc+*x as u32)%256) as u8;
        let mut to_send = [0u8; Data::SIZE+3];
        to_send[..Data::SIZE+2].copy_from_slice(&data_buf[..{Data::SIZE+2}]);
        to_send[Data::SIZE+2]=chksm;
        self.s.write(&to_send)?;
        let mut readen=[0u8; <((u8, Res), u8)>::SIZE];
        self.s.read(&mut readen)?;
        let chksm = (readen[..(Res::SIZE+1)].iter().fold(0u32, |acc, x|acc+*x as u32)%256) as u8;
        if *readen.last().unwrap()!=chksm{
            println!("invalid checksum {} {chksm}", *readen.last().unwrap());
            return Err(SerialError::Undefined);
        }
        let result =<((u8, Res), u8)>::from_u8(readen);
        if result.0.0!=self.address{
            println!("invalid address");
            return Err(SerialError::Undefined);
        } 
        Ok(result.0.1)
    }
}

impl<T: Serial> Servo42C<T>{
    /**
    read the encoder value (the motor should be calibrated)
    returns (carry, value)  where
    - carry is the value of the encoder (giri?)
    - current value of the encoder (fase)
     */
    pub fn read_encoder_value(&mut self)->(i32, u16){
        self.send_cmd(0x30, ()).unwrap()
    }

    /**
     Read the number of pulses received.
    */
    pub fn read_recived_pulses(&mut self)->i32{
        self.send_cmd(0x33, ()).unwrap()
    }

    /**recived_pulses
     * read the error of the motor shaft angle
     The error is the difference between the angle you want to control 
     minus  the  real-time  angle  of  the  motor,  0~FFFF  corresponds  to 
     0~360°. 
     for  example,  when  the  angle  error  is  1°,  the  return  error  is 
     65536/360= 182.444, and so on.
     */
    pub fn read_error(&mut self)->i32{
        self.send_cmd(0x39, ()).unwrap()
    }

}


#[cfg(test)]
mod tests{
    use serial::test::SerialTest;
    use crate::Servo42C;
    macro_rules! test_motor {
        ($name:ident, ($($val:literal) *)->($($ret:literal) *)) => {
            #[test]
            fn $name(){
                let mut s = SerialTest::default();
                s.add_response(vec![$($val),*], vec![$($ret),*]);
                let mut servo=Servo42C{address: 0xe0, s};
                servo.$name();
            }
        };
    }
    
    test_motor!(read_encoder_value, (0xe0 0x30 0x10)->(0xe0 00 00 00 00 0x40 00 0x20));
    
}
