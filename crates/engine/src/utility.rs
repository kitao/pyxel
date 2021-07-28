use crate::rectarea::RectArea;

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

#[inline]
pub fn data_value<T: Copy>(data: &Vec<Vec<T>>, x: i32, y: i32) -> T {
    data[y as usize][x as usize]
}

#[inline]
pub fn data_value_with_check<T: Copy + Default>(
    data: &Vec<Vec<T>>,
    rect: RectArea,
    x: i32,
    y: i32,
) -> T {
    if rect.contains(x, y) {
        data[y as usize][x as usize]
    } else {
        T::default()
    }
}

#[inline]
pub fn set_data_value<T: Copy>(data: &mut Vec<Vec<T>>, x: i32, y: i32, value: T) {
    data[y as usize][x as usize] = value;
}

#[inline]
pub fn set_data_value_with_check<T: Copy>(
    data: &mut Vec<Vec<T>>,
    rect: RectArea,
    x: i32,
    y: i32,
    value: T,
) {
    if rect.contains(x, y) {
        data[y as usize][x as usize] = value;
    }
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

    #[test]
    fn data_value_() {
        let data = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(data_value(&data, 0, 0), 1);
        assert_eq!(data_value(&data, 1, 0), 2);
        assert_eq!(data_value(&data, 0, 1), 3);
        assert_eq!(data_value(&data, 1, 1), 4);
    }

    #[test]
    fn data_value_with_check_() {
        let data = vec![vec![1, 2], vec![3, 4]];
        let rect = RectArea::new(0, 0, 2, 2);

        assert_eq!(data_value_with_check(&data, rect, 0, 0), 1);
        assert_eq!(data_value_with_check(&data, rect, 1, 0), 2);
        assert_eq!(data_value_with_check(&data, rect, 0, 1), 3);
        assert_eq!(data_value_with_check(&data, rect, 1, 1), 4);

        assert_eq!(data_value_with_check(&data, rect, -1, 0), 0);
        assert_eq!(data_value_with_check(&data, rect, 0, -1), 0);
        assert_eq!(data_value_with_check(&data, rect, 2, 0), 0);
        assert_eq!(data_value_with_check(&data, rect, 0, 2), 0);
    }

    #[test]
    fn set_data_value_() {
        let mut data = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(data_value(&data, 0, 0), 1);
        assert_eq!(data_value(&data, 1, 0), 2);
        assert_eq!(data_value(&data, 0, 1), 3);
        assert_eq!(data_value(&data, 1, 1), 4);

        set_data_value(&mut data, 0, 0, 5);
        set_data_value(&mut data, 1, 0, 6);
        set_data_value(&mut data, 0, 1, 7);
        set_data_value(&mut data, 1, 1, 8);

        assert_eq!(data_value(&data, 0, 0), 5);
        assert_eq!(data_value(&data, 1, 0), 6);
        assert_eq!(data_value(&data, 0, 1), 7);
        assert_eq!(data_value(&data, 1, 1), 8);
    }

    #[test]
    fn set_data_value_with_check_() {
        let mut data = vec![vec![1, 2], vec![3, 4]];
        let rect = RectArea::new(0, 0, 2, 2);

        assert_eq!(data_value(&data, 0, 0), 1);
        assert_eq!(data_value(&data, 1, 0), 2);
        assert_eq!(data_value(&data, 0, 1), 3);
        assert_eq!(data_value(&data, 1, 1), 4);

        set_data_value(&mut data, 0, 0, 5);
        set_data_value(&mut data, 1, 0, 6);
        set_data_value(&mut data, 0, 1, 7);
        set_data_value(&mut data, 1, 1, 8);

        set_data_value_with_check(&mut data, rect, -1, 0, 5);
        set_data_value_with_check(&mut data, rect, 0, -1, 6);
        set_data_value_with_check(&mut data, rect, 0, 1, 7);
        set_data_value_with_check(&mut data, rect, 1, 1, 8);

        assert_eq!(data_value(&data, 0, 0), 5);
        assert_eq!(data_value(&data, 1, 0), 6);
        assert_eq!(data_value(&data, 0, 1), 7);
        assert_eq!(data_value(&data, 1, 1), 8);
    }
}
