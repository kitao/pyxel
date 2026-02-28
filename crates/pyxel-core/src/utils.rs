macro_rules! string_loop {
    ($index: ident, $piece: ident, $string: ident, $step: expr, $block: block) => {
        for $index in 0..($string.len() / $step) {
            let __index = $index * $step;
            let $piece = $string[__index..__index + $step].to_string();
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

pub fn simplify_string(string: &str) -> String {
    remove_whitespace(string).to_ascii_lowercase()
}

pub fn parse_hex_string(string: &str) -> Result<u32, &str> {
    let string = string.to_ascii_lowercase();
    let mut result: u32 = 0;

    for c in string.chars() {
        result *= 0x10;
        if c.is_ascii_digit() {
            result += c as u32 - '0' as u32;
        } else if ('a'..='f').contains(&c) {
            result += 10 + c as u32 - 'a' as u32;
        } else {
            return Err("invalid hex string");
        }
    }

    Ok(result)
}

pub fn add_file_extension(filename: &str, ext: &str) -> String {
    if filename.to_lowercase().ends_with(ext) {
        filename.to_string()
    } else {
        filename.to_string() + ext
    }
}

pub fn compress_vec<T: PartialEq + Clone>(vec: &[T]) -> Vec<T> {
    assert!(!vec.is_empty());
    let mut new_vec = vec.to_vec();
    let mut new_len = new_vec.len();

    for i in (1..new_vec.len()).rev() {
        if new_vec[i] == new_vec[i - 1] {
            new_len = i;
        } else {
            break;
        }
    }

    new_vec.truncate(new_len);
    new_vec
}

pub fn compress_vec2<T: PartialEq + Clone>(vec: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!vec.is_empty());
    compress_vec(vec)
        .iter()
        .map(|inner_vec| compress_vec(inner_vec))
        .collect::<Vec<_>>()
}

pub fn expand_vec<T: Clone + Default>(vec: &[T], new_len: usize) -> Vec<T> {
    assert!(!vec.is_empty());
    let mut new_vec = vec.to_vec();

    if let Some(last) = new_vec.last().cloned() {
        new_vec.resize_with(new_len, move || last.clone());
    }

    new_vec
}

pub fn expand_vec2<T: Clone + Default>(
    vec: &[Vec<T>],
    new_outer_len: usize,
    new_inner_len: usize,
) -> Vec<Vec<T>> {
    assert!(!vec.is_empty());
    let new_vec = vec
        .iter()
        .map(|inner_vec| expand_vec(inner_vec, new_inner_len))
        .collect::<Vec<_>>();

    expand_vec(&new_vec, new_outer_len)
}

pub fn trim_empty_vecs<T: Clone>(vecs: &[Vec<T>]) -> Vec<Vec<T>> {
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

    // f32_to_i32 / f32_to_u32

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

    // String functions

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace(" a\n b\r c\t d "), "abcd");
        assert_eq!(remove_whitespace(""), "");
        assert_eq!(remove_whitespace("abc"), "abc");
    }

    #[test]
    fn test_simplify_string() {
        assert_eq!(simplify_string(" 0\n 1\r 2\t 3 A\n b\r c\t d "), "0123abcd");
        assert_eq!(simplify_string("ABC"), "abc");
        assert_eq!(simplify_string(""), "");
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
    fn test_add_file_extension() {
        assert_eq!(add_file_extension("test", ".png"), "test.png");
        assert_eq!(add_file_extension("test.png", ".png"), "test.png");
        assert_eq!(add_file_extension("test.PNG", ".png"), "test.PNG");
        assert_eq!(add_file_extension("test.txt", ".png"), "test.txt.png");
    }

    // Macros

    #[test]
    fn test_string_loop() {
        let s = "ABCDEF";
        let mut result = Vec::new();
        string_loop!(_i, piece, s, 2, {
            result.push(piece);
        });
        assert_eq!(result, vec!["AB", "CD", "EF"]);
    }

    // Vec compress/expand/trim

    #[test]
    fn test_compress_vec() {
        assert_eq!(compress_vec(&[1, 2, 2, 3, 3, 3]), vec![1, 2, 2, 3]);
        assert_eq!(compress_vec(&[4, 4, 4, 4, 4]), vec![4]);
        assert_eq!(compress_vec(&[2]), vec![2]);
        assert_eq!(compress_vec(&[1, 2, 3]), vec![1, 2, 3]);
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
    fn test_trim_empty_vecs() {
        assert_eq!(
            trim_empty_vecs(&[vec![1, 2], vec![], vec![], vec![3, 4], vec![], vec![]]),
            vec![vec![1, 2], vec![], vec![], vec![3, 4]]
        );
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(trim_empty_vecs::<i32>(&[vec![], vec![]]), empty);
        assert_eq!(trim_empty_vecs(&[vec![1], vec![2]]), vec![vec![1], vec![2]]);
        assert_eq!(trim_empty_vecs::<i32>(&[vec![]]), empty);
        assert_eq!(trim_empty_vecs(&[vec![1, 2]]), vec![vec![1, 2]]);
    }
}
