pub fn remove_whitespace(s: &str) -> String {
    s.replace(&[' ', '\n', '\r', '\t'][..], "")
}

pub fn simplify_string(s: &str) -> String {
    remove_whitespace(s).to_ascii_lowercase()
}

pub fn parse_hex_string(s: &str) -> Option<u32> {
    let s = s.to_ascii_lowercase();
    let mut result: u32 = 0;

    for c in s.chars() {
        result *= 0x10;

        if c >= '0' && c <= '9' {
            result += c as u32 - '0' as u32;
        } else if c >= 'a' && c <= 'f' {
            result += 10 + c as u32 - 'a' as u32;
        } else {
            return None;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_whitespace_() {
        assert_eq!(remove_whitespace(" a\n b\r c\t d "), "abcd")
    }

    #[test]
    fn simplify_string_() {
        assert_eq!(simplify_string(" 0\n 1\r 2\t 3 A\n b\r c\t d "), "0123abcd");
    }

    #[test]
    fn parse_hex_string_() {
        assert_eq!(parse_hex_string("100"), Some(256));
        assert_eq!(parse_hex_string("a2"), Some(162));
        assert_eq!(parse_hex_string("BC"), Some(188));
        assert_eq!(parse_hex_string(" "), None);
    }
}
