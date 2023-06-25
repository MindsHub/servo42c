use std::io::Read;

use super::serialtrait::*;
use serial2::*;

impl Serial for SerialPort{
    fn read(&mut self, buf: &mut [u8])->Result<(), SerialError> {
        self.read_exact(buf).map_err(|_| SerialError::Undefined)
    }

    fn write(&mut self, buf: &[u8])->Result<(), SerialError> {
        self.write_all(buf).map_err(|_| SerialError::Undefined)
    }
}