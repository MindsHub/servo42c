use crate::serialtrait::{Serial, SerialError};
use embedded_hal::serial::*;
impl<S> Serial for S
where
    S: Read<u8> + Write<u8>,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<(), crate::serialtrait::SerialError> {
        let mut readen = 0usize;
        let mut failed = 0usize;
        loop {
            if let Ok(x) = self.read() {
                buf[readen] = x;
                readen += 1;
            } else {
                failed += 1;
            }
            if failed > 65534 {
                return Err(SerialError::Undefined);
            }
            if readen >= buf.len() {
                break;
            }
        }
        return Ok(());
    }

    fn write(&mut self, buf: &[u8]) -> Result<(), crate::serialtrait::SerialError> {
        for x in buf {
            while let Err(_) = self.write(*x) {}
        }
        //while let Err(_) = self.flush(){};
        Ok(())
    }
}
