use std::collections::HashMap;

use crate::serialtrait::SerialError;

use super::serialtrait::Serial;

#[derive(Default, Debug)]
pub struct SerialTest {
    map: HashMap<Vec<u8>, Vec<u8>>,
    response: Option<Vec<u8>>,
}

impl SerialTest {
    pub fn add_response(&mut self, w: Vec<u8>, r: Vec<u8>) {
        println!("{w:?}->{r:?}");
        self.map.insert(w, r);
    }
}

impl Serial for SerialTest {
    fn read(&mut self, buf: &mut [u8]) -> Result<(), crate::serialtrait::SerialError> {
        if let Some(mut x) = self.response.take() {
            println!("read: {x:?} {}->{}", x.len(), buf.len());
            if x.len() != buf.len() {
                return Err(SerialError::Undefined);
            }
            buf.swap_with_slice(x.as_mut_slice());
            Ok(())
        } else {
            Err(SerialError::Undefined)
        }
    }
    fn write(&mut self, buf: &[u8]) -> Result<(), crate::serialtrait::SerialError> {
        println!("send: {buf:?}");
        if let Some(val) = self.map.get(buf) {
            self.response = Some(val.clone());
            Ok(())
        } else {
            Err(SerialError::Undefined)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serialtrait::Serial;

    use super::SerialTest;

    #[test]
    fn check_serial_test() {
        let mut t = SerialTest::default();
        t.add_response(vec![0xEF], vec![0x01]);
        t.write(vec![0xEF].as_mut_slice()).unwrap();
    }
}
