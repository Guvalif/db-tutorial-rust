use std::cmp::min;
use std::fmt;
use std::str::from_utf8;

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

#[derive(Debug)]
#[repr(C)]
pub struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

fn string_to_fixedbytes<const N: usize>(string: &str) -> [u8; N] {
    let mut bytes = [b'\0'; N];
    let ptr = string.as_ptr();

    unsafe {
        for i in 0..min(string.len(), N) {
            bytes[i] = *ptr.add(i);
        }
    }

    bytes
}

impl Row {
    pub fn new(id: u32, username: &str, email: &str) -> Row {
        Row {
            id,
            username: string_to_fixedbytes(username),
            email: string_to_fixedbytes(email),
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Row {{ id: {}, username: \"{}\", email: \"{}\" }}",
            self.id,
            from_utf8(&self.username).unwrap(),
            from_utf8(&self.email).unwrap()
        )
    }
}
