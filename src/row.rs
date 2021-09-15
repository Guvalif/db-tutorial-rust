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
    pub fn new(id: u32, username: &str, email: &str) -> Result<Row, String> {
        if username.len() > COLUMN_USERNAME_SIZE {
            return Err("Error: 'username' is too long.".to_owned());
        }

        if email.len() > COLUMN_EMAIL_SIZE {
            return Err("Error: 'email' is too long.".to_owned());
        }

        Ok(Row {
            id,
            username: string_to_fixedbytes(username),
            email: string_to_fixedbytes(email),
        })
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Row {{ id: {}, username: \"{}\", email: \"{}\" }}",
            self.id,
            from_utf8(&self.username).unwrap().trim_matches('\0'),
            from_utf8(&self.email).unwrap().trim_matches('\0')
        )
    }
}
