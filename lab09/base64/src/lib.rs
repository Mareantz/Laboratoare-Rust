/// Encodes a byte slice into a Base64 string.
///
/// This function takes a byte slice as input and returns a Base64-encoded string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use base64::encode;
///
/// let input = "hello";
/// let output = encode(&input.bytes().collect::<Vec<u8>>());
///
/// assert_eq!(output, "aGVsbG8=");
/// ```
///
/// Note that `input.bytes().collect::<Vec<u8>>()` is used to convert the string into a byte slice, which is what the `encode` function expects.

pub fn encode(input: &[u8]) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < input.len() {
        let mut chunk = [0; 3];
        let mut j = 0;
        while j < 3 && i + j < input.len() {
            chunk[j] = input[i + j];
            j += 1;
        }
        let mut bits = 0;
        for k in 0..j {
            bits = bits << 8;
            bits += chunk[k] as u32;
        }
        bits <<= (3 - j) * 8;
        for k in 0..j + 1 {
            let index = (bits >> (18 - k * 6)) & 0x3f;
            let ascii_index = match index {
                0..=25 => index + 65,
                26..=51 => index + 71,
                52..=61 => index - 4,
                62 => 43,
                63 => 47,
                _ => panic!("Invalid index {}", index),
            };
            output += &format!("{}", ascii_index as u8 as char);
        }
        while output.len() % 4 != 0 {
            output += "=";
        }
        i += 3;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("Man".as_bytes()), "TWFu");
        assert_eq!(
            encode("Many hands make light work.".as_bytes()),
            "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu"
        );
        assert_eq!(encode("".as_bytes()), "");
        assert_eq!(encode("f".as_bytes()), "Zg==");
        assert_eq!(encode("fo".as_bytes()), "Zm8=");
        assert_eq!(encode("foo".as_bytes()), "Zm9v");
        assert_eq!(encode("foob".as_bytes()), "Zm9vYg==");
        assert_eq!(encode("fooba".as_bytes()), "Zm9vYmE=");
        assert_eq!(encode("foobar".as_bytes()), "Zm9vYmFy");
    }
}
