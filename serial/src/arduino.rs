use crate::serialtrait::{Serial, SerialError};
use embedded_hal::serial::*;
impl<S: Read<u8>+ Write<u8>> Serial
    for S where
    //S: Read<u8>+ Write<u8>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<(), crate::serialtrait::SerialError> {
        return Ok(());
        /*let mut readen = 0usize;
        let mut failed = 0usize;
        loop{
            if let Ok(x) = self.read() {
                buf[readen]=x;
                readen += 1;
            } else {
                failed += 1;
            }
            if failed > 10 {
                return Err(SerialError::Undefined);
            }
            if readen >= buf.len() {
                break;
            }
        }*/
        
    }
 
    fn write(&mut self, buf: &[u8]) -> Result<(), crate::serialtrait::SerialError> {
        //0xe0 0x30 0x10
        for x in buf{
            send_byte(self, *x);
        }
        /*send_byte(self, buf[0]);
        send_byte(self, buf[1]);
        send_byte(self, buf[2]);
        send_byte(self, 0x10);
        */let _ = self.flush();
        Ok(())
    }
}
fn send_byte<T: Write<u8>>(s: &mut T, c: u8){
    for _ in 1..=51000u32{
        let _ =s.flush();
    };
    let _ = s.write(c);
}