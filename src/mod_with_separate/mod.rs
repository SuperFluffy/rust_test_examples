use std::string::FromUtf8Error;

pub fn buf_to_string(buf: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(buf.to_vec())
}

#[cfg(test)]
mod tests;
