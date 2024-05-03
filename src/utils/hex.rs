use super::Result;
use std::fmt::Write;
use std::sync::Arc;

/// Convert bytes into hexastring
#[tracing::instrument(skip_all, err)]
pub fn encode(bytes: &[u8]) -> Result<Arc<str>> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b)?;
    }
    Ok(Arc::from(s.as_str()))
}

/// Convert hexastring into bytes
#[tracing::instrument(skip_all, err)]
pub fn decode(s: &str) -> Result<Arc<[u8]>> {
    let mut v: Vec<u8> = Vec::new();
    for i in (0..s.len()).step_by(2) {
        let b = u8::from_str_radix(&s[i..i + 2], 16)?;
        v.push(b);
    }
    Ok(Arc::from(v.as_slice()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_ok() {
        let bytes = [0x1e; 4];
        let expected = "1e1e1e1e";

        let hex_str = encode(&bytes).unwrap();

        assert_eq!(&*hex_str, expected);
    }

    #[test]
    fn encode_fail() {
        let bytes = [0x11; 4];
        let wrong = "1e1e1e1e";

        let hex_str = encode(&bytes).unwrap();

        assert_ne!(&*hex_str, wrong);
    }

    #[test]
    fn decode_ok() {
        let hex_str = "1e1e1e1e";
        let expected = [0x1e; 4];

        let bytes = decode(hex_str).unwrap();

        assert_eq!(*bytes, expected);
    }

    #[test]
    fn decode_fail() {
        let hex_str = "1e1e1e1e";
        let wrong = [0x11; 4];

        let bytes = decode(hex_str).unwrap();

        assert_ne!(*bytes, wrong);
    }
}
