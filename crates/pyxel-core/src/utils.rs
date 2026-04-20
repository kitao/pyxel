macro_rules! string_loop {
    ($i: ident, $piece: ident, $s: ident, $step: expr, $block: block) => {
        for $i in 0..($s.len() / $step) {
            let _macro_index = $i * $step;
            let $piece = &$s[_macro_index.._macro_index + $step];
            $block
        }
    };
}

macro_rules! repeat_extend {
    ($container:expr, $value:expr, $count:expr) => {
        $container.extend(std::iter::repeat($value).take($count as usize));
    };
}

pub fn f32_to_i32(x: f32) -> i32 {
    x.round() as i32
}

pub fn f32_to_u32(x: f32) -> u32 {
    x.round() as u32
}

pub fn remove_whitespace(string: &str) -> String {
    string.replace(&[' ', '\n', '\r', '\t'][..], "")
}

pub fn compact_ascii_lower(string: &str) -> String {
    string
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

pub fn parse_hex_string(string: &str) -> Result<u32, &str> {
    let mut result: u32 = 0;
    for c in string.bytes() {
        result *= 0x10;
        match c {
            b'0'..=b'9' => result += (c - b'0') as u32,
            b'a'..=b'f' => result += 10 + (c - b'a') as u32,
            b'A'..=b'F' => result += 10 + (c - b'A') as u32,
            _ => return Err("invalid hex string"),
        }
    }
    Ok(result)
}

pub fn add_file_extension(filename: &str, ext: &str) -> String {
    if filename.to_lowercase().ends_with(ext) {
        filename.to_string()
    } else {
        format!("{filename}{ext}")
    }
}

pub fn compress_vec<T: PartialEq + Clone>(vec: &[T]) -> Vec<T> {
    assert!(!vec.is_empty());
    let last = vec.last().unwrap();
    let new_len = vec.iter().rposition(|v| v != last).map_or(1, |i| i + 2);
    vec[..new_len].to_vec()
}

pub fn compress_vec2<T: PartialEq + Clone>(vec: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!vec.is_empty());
    compress_vec(vec)
        .iter()
        .map(|inner_vec| compress_vec(inner_vec))
        .collect()
}

pub fn expand_vec<T: Clone>(vec: &[T], new_len: usize) -> Vec<T> {
    assert!(!vec.is_empty());
    let mut new_vec = vec.to_vec();
    let last = new_vec.last().cloned().unwrap();
    new_vec.resize(new_len, last);
    new_vec
}

pub fn expand_vec2<T: Clone>(
    vec: &[Vec<T>],
    new_outer_len: usize,
    new_inner_len: usize,
) -> Vec<Vec<T>> {
    assert!(!vec.is_empty());
    let new_vec: Vec<_> = vec
        .iter()
        .map(|inner_vec| expand_vec(inner_vec, new_inner_len))
        .collect();

    expand_vec(&new_vec, new_outer_len)
}

pub fn trim_empty_vec<T: Clone>(vecs: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut vecs = vecs.to_vec();
    let new_len = vecs
        .iter()
        .rev()
        .position(|vec| !vec.is_empty())
        .map_or(0, |i| vecs.len() - i);

    vecs.truncate(new_len);
    vecs
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── f32_to_i32 / f32_to_u32 ──

    #[test]
    fn test_f32_to_i32() {
        assert_eq!(f32_to_i32(0.1), 0);
        assert_eq!(f32_to_i32(0.49), 0);
        assert_eq!(f32_to_i32(0.5), 1);
        assert_eq!(f32_to_i32(1.49), 1);
        assert_eq!(f32_to_i32(-0.1), 0);
        assert_eq!(f32_to_i32(-0.49), 0);
        assert_eq!(f32_to_i32(-0.5), -1);
        assert_eq!(f32_to_i32(-1.49), -1);
    }

    #[test]
    fn test_f32_to_u32() {
        assert_eq!(f32_to_u32(0.1), 0);
        assert_eq!(f32_to_u32(0.49), 0);
        assert_eq!(f32_to_u32(0.5), 1);
        assert_eq!(f32_to_u32(1.49), 1);
        assert_eq!(f32_to_u32(-0.1), 0);
        assert_eq!(f32_to_u32(-3.0), 0);
    }

    // ── String functions ──

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace(" a\n b\r c\t d "), "abcd");
        assert_eq!(remove_whitespace(""), "");
        assert_eq!(remove_whitespace("abc"), "abc");
    }

    #[test]
    fn test_compact_ascii_lower() {
        assert_eq!(compact_ascii_lower(" 0\n 1\r 2\t 3 A\n b\r c\t d "), "0123abcd");
        assert_eq!(compact_ascii_lower("ABC"), "abc");
        assert_eq!(compact_ascii_lower(""), "");
    }

    #[test]
    fn test_parse_hex_string() {
        assert_eq!(parse_hex_string("0"), Ok(0));
        assert_eq!(parse_hex_string("100"), Ok(256));
        assert_eq!(parse_hex_string("a2"), Ok(162));
        assert_eq!(parse_hex_string("BC"), Ok(188));
        assert_eq!(parse_hex_string("ff"), Ok(255));
        assert_eq!(parse_hex_string("FFFF"), Ok(0xFFFF));
        assert_eq!(parse_hex_string(" "), Err("invalid hex string"));
        assert_eq!(parse_hex_string("g0"), Err("invalid hex string"));
    }

    #[test]
    fn test_parse_hex_string_edge_cases() {
        // Empty string
        assert_eq!(parse_hex_string(""), Ok(0));
        // Single digit
        assert_eq!(parse_hex_string("F"), Ok(15));
        // u32 max value
        assert_eq!(parse_hex_string("FFFFFFFF"), Ok(u32::MAX));
        // Mixed case in same string
        assert_eq!(parse_hex_string("aB"), Ok(0xAB));
    }

    #[test]
    fn test_add_file_extension() {
        assert_eq!(add_file_extension("test", ".png"), "test.png");
        assert_eq!(add_file_extension("test.png", ".png"), "test.png");
        assert_eq!(add_file_extension("test.PNG", ".png"), "test.PNG");
        assert_eq!(add_file_extension("test.txt", ".png"), "test.txt.png");
    }

    #[test]
    fn test_add_file_extension_edge_cases() {
        // Empty filename
        assert_eq!(add_file_extension("", ".png"), ".png");
        // Partial extension match (should NOT match)
        assert_eq!(add_file_extension("test.pn", ".png"), "test.pn.png");
        // Extension only
        assert_eq!(add_file_extension(".png", ".png"), ".png");
    }

    // ── Macros ──

    #[test]
    fn test_string_loop() {
        let s = "ABCDEF";
        let mut result = Vec::new();
        string_loop!(_i, piece, s, 2, {
            result.push(piece);
        });
        assert_eq!(result, vec!["AB", "CD", "EF"]);
    }

    #[test]
    fn test_repeat_extend() {
        let mut v = vec![1, 2];
        repeat_extend!(v, 0, 3);
        assert_eq!(v, vec![1, 2, 0, 0, 0]);

        let mut v: Vec<i32> = Vec::new();
        repeat_extend!(v, 42, 0);
        assert!(v.is_empty());
    }

    // ── Vec compress/expand/trim ──

    #[test]
    fn test_compress_vec() {
        assert_eq!(compress_vec(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 2, 3]);
        assert_eq!(compress_vec(&[4, 4, 4, 4, 4]), vec![4]);
        assert_eq!(compress_vec(&[2]), vec![2]);
        assert_eq!(compress_vec(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_compress_vec_two_distinct() {
        assert_eq!(compress_vec(&[1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_compress_vec2() {
        assert_eq!(
            compress_vec2(&[vec![1, 1, 2], vec![2, 2, 2], vec![3, 3, 3], vec![3, 3, 3]]),
            vec![vec![1, 1, 2], vec![2], vec![3]]
        );
        assert_eq!(compress_vec2(&[vec![2]]), vec![vec![2]]);
    }

    #[test]
    fn test_expand_vec() {
        assert_eq!(expand_vec(&[1, 2, 3], 5), vec![1, 2, 3, 3, 3]);
        assert_eq!(expand_vec(&[4], 3), vec![4, 4, 4]);
        assert_eq!(expand_vec(&[1, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn test_expand_vec_same_length() {
        assert_eq!(expand_vec(&[1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_expand_vec2() {
        assert_eq!(
            expand_vec2(&[vec![1, 2], vec![3]], 4, 3),
            vec![vec![1, 2, 2], vec![3, 3, 3], vec![3, 3, 3], vec![3, 3, 3]]
        );
        assert_eq!(
            expand_vec2(
                &[
                    vec![1, 2, 4],
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                    vec![10, 11, 12]
                ],
                3,
                2
            ),
            vec![vec![1, 2], vec![4, 5], vec![7, 8]]
        );
    }

    #[test]
    fn test_expand_vec2_same_dimensions() {
        let input = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(expand_vec2(&input, 2, 2), input);
    }

    #[test]
    fn test_trim_empty_vec() {
        assert_eq!(
            trim_empty_vec(&[vec![1, 2], vec![], vec![], vec![3, 4], vec![], vec![]]),
            vec![vec![1, 2], vec![], vec![], vec![3, 4]]
        );
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(trim_empty_vec::<i32>(&[vec![], vec![]]), empty);
        assert_eq!(trim_empty_vec(&[vec![1], vec![2]]), vec![vec![1], vec![2]]);
        assert_eq!(trim_empty_vec::<i32>(&[vec![]]), empty);
        assert_eq!(trim_empty_vec(&[vec![1, 2]]), vec![vec![1, 2]]);
    }
}
