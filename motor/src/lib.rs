#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use serial::serialtrait::{Serial, Sendable, SerialError};

pub struct Servo42C<T: Serial>{
    address: u8,
    s: T,
}

impl<T: Serial> Servo42C<T>{
    pub fn send_cmd<Data: Sendable<DATAS>, Res: Sendable<RESSIZE>, const RESSIZE: usize, const DATAS: usize>(&mut self, code: u8, data: Data)->Result<Res, SerialError>
        where [(); DATAS+2]:
        {
        let buffer: [u8; 2] = (self.address, code).as_u8();
        let data: [u8; DATAS+2] =(buffer, data).as_u8() ;

        /*
        buffer[0]=self.address;
        buffer[1]=code;
        buffer[2..(DATASIZE + 2)].copy_from_slice(&data[..mem::size_of::<Y>()]);
        buffer[2+DATASIZE]=buffer[..DATASIZE+1].iter().sum();
        self.s.write(&buffer[..(3+DATASIZE)])?;
        self.s.read(&mut buffer[..(RESSIZE+2)])?;
        if buffer[..DATASIZE+1].iter().sum::<u8>()==buffer[DATASIZE+2] {
            let res=buffer[1..(RESSIZE+1)].try_into().map_err(|_| SerialError::Undefined)?;
            let res = Res::from_u8(res);
            Ok(res)
        }else{
            Err(SerialError::Undefined)
        }*/
        todo!()
    }
}

impl<T: Serial> Servo42C<T>{
    fn read_encoder_value(&mut self){
        //let y = self.send_cmd(30, ());
    }
}


#[cfg(test)]
mod tests{
    use serial::test::SerialTest;

    use crate::Servo42C;

    #[test]
    fn check_serial_test(){
        let s = SerialTest::default();
        let servo=Servo42C { address: 0xe0, s};
        //servo.send_cmd(code, data)
    }
}
