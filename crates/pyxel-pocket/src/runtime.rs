use std::ffi::CString;
use std::sync::{Mutex, MutexGuard};

use crate::{ffi, module};

static RUNTIME_LOCK: Mutex<()> = Mutex::new(());

const PYTHON_COMPAT_SOURCE: &str = r#"
from collections import deque as __pyxel_pocket_deque

def __pyxel_pocket_deque_getitem(self, index):
    length = len(self)
    if index < 0:
        index += length
    if index < 0 or index >= length:
        raise IndexError("deque index out of range")
    return self._data[(self._head + index) % self._capacity]

__pyxel_pocket_deque.__getitem__ = __pyxel_pocket_deque_getitem
del __pyxel_pocket_deque_getitem
del __pyxel_pocket_deque
"#;

pub struct Runtime {
    _guard: MutexGuard<'static, ()>,
}

impl Runtime {
    pub fn new() -> Self {
        let guard = RUNTIME_LOCK.lock().expect("PocketPy runtime lock poisoned");
        unsafe {
            ffi::py_initialize();
        }
        module::register();
        install_python_compat();
        Self { _guard: guard }
    }

    pub fn exec_source(&self, source: &str, filename: &str) -> Result<(), String> {
        let source = normalize_source(source);
        let source = CString::new(source).map_err(|_| "source contains NUL byte".to_owned())?;
        let filename =
            CString::new(filename).map_err(|_| "filename contains NUL byte".to_owned())?;
        let ok = unsafe {
            ffi::py_exec(
                source.as_ptr(),
                filename.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                std::ptr::null_mut(),
            )
        };
        if ok {
            Ok(())
        } else {
            unsafe {
                ffi::py_printexc();
            }
            Err(format!("PocketPy failed to execute {filename:?}"))
        }
    }
}

fn install_python_compat() {
    let source = CString::new(PYTHON_COMPAT_SOURCE).expect("compat source contains NUL byte");
    let filename = CString::new("<pyxel-pocket-compat>").unwrap();
    let ok = unsafe {
        ffi::py_exec(
            source.as_ptr(),
            filename.as_ptr(),
            ffi::py_CompileMode_EXEC_MODE,
            std::ptr::null_mut(),
        )
    };
    if !ok {
        unsafe {
            ffi::py_printexc();
        }
        panic!("failed to install PocketPy compatibility patches");
    }
}

fn normalize_source(source: &str) -> String {
    let mut normalized = String::with_capacity(source.len());
    let mut open_delimiters = 0i32;
    let mut unpack_index = 0usize;
    let mut slice_assign_index = 0usize;

    for line in source.lines() {
        if let Some(expanded) = nested_list_comprehension_assignment(line) {
            normalized.push_str(&expanded);
            continue;
        }

        if let Some((indent, target, value)) = whole_slice_assignment(line) {
            let temp = format!("__pyxel_pocket_slice_assign_{slice_assign_index}");
            normalized.push_str(&indent);
            normalized.push_str(&temp);
            normalized.push_str(" = ");
            normalized.push_str(&value);
            normalized.push('\n');
            normalized.push_str(&indent);
            normalized.push_str(&target);
            normalized.push_str(".clear()\n");
            normalized.push_str(&indent);
            normalized.push_str(&target);
            normalized.push_str(".extend(");
            normalized.push_str(&temp);
            normalized.push_str(")\n");
            slice_assign_index += 1;
            continue;
        }

        if let Some((indent, target, iterable)) = unpacking_for_loop(line) {
            let temp = format!("__pyxel_pocket_unpack_{unpack_index}");
            normalized.push_str(&indent);
            normalized.push_str("for ");
            normalized.push_str(&temp);
            normalized.push_str(" in ");
            normalized.push_str(&iterable);
            normalized.push_str(":\n");
            normalized.push_str(&indent);
            normalized.push_str("    ");
            normalized.push_str(&target);
            normalized.push_str(" = ");
            normalized.push_str(&temp);
            normalized.push('\n');
            unpack_index += 1;
            continue;
        }

        let trimmed = line.trim_start();
        let joins_leading_operator = open_delimiters > 0 && is_leading_operator(trimmed);
        let joins_adjacent_string = open_delimiters > 0
            && is_leading_string_literal(trimmed)
            && matches!(last_significant_char(&normalized), Some('\'' | '"'));

        if joins_leading_operator || joins_adjacent_string {
            if normalized.ends_with('\n') {
                normalized.pop();
            }
            if joins_adjacent_string {
                normalized.push_str(" + ");
            } else {
                normalized.push(' ');
            }
            normalized.push_str(trimmed);
            normalized.push('\n');
        } else {
            normalized.push_str(line);
            normalized.push('\n');
        }

        open_delimiters = (open_delimiters + delimiter_delta(line)).max(0);
    }

    if !source.ends_with('\n') && normalized.ends_with('\n') {
        normalized.pop();
    }
    normalized
}

fn nested_list_comprehension_assignment(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let (target, value) = trimmed.split_once(" = ")?;
    let body = value.strip_prefix('[')?.strip_suffix(']')?;
    let (item, rest) = body.split_once(" for ")?;
    let (first_for, second_for) = rest.split_once(" for ")?;
    if second_for.contains(" for ") || second_for.contains(" if ") {
        return None;
    }

    let (first_var, first_iter) = first_for.split_once(" in ")?;
    let (second_var, second_iter) = second_for.split_once(" in ")?;

    let mut expanded = String::new();
    expanded.push_str(indent);
    expanded.push_str(target.trim());
    expanded.push_str(" = []\n");
    expanded.push_str(indent);
    expanded.push_str("for ");
    expanded.push_str(first_var.trim());
    expanded.push_str(" in ");
    expanded.push_str(first_iter.trim());
    expanded.push_str(":\n");
    expanded.push_str(indent);
    expanded.push_str("    for ");
    expanded.push_str(second_var.trim());
    expanded.push_str(" in ");
    expanded.push_str(second_iter.trim());
    expanded.push_str(":\n");
    expanded.push_str(indent);
    expanded.push_str("        ");
    expanded.push_str(target.trim());
    expanded.push_str(".append(");
    expanded.push_str(item.trim());
    expanded.push_str(")\n");
    Some(expanded)
}

fn whole_slice_assignment(line: &str) -> Option<(String, String, String)> {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let (target, value) = trimmed.split_once("[:] = ")?;
    Some((
        indent.to_owned(),
        target.trim().to_owned(),
        value.trim().to_owned(),
    ))
}

fn unpacking_for_loop(line: &str) -> Option<(String, String, String)> {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let rest = trimmed.strip_prefix("for ")?;
    let (target, iterable) = rest.split_once(" in ")?;
    let iterable = iterable.strip_suffix(':')?;
    if !target.contains('(') && !target.contains(')') {
        return None;
    }

    Some((
        indent.to_owned(),
        target.trim().to_owned(),
        iterable.trim().to_owned(),
    ))
}

fn is_leading_operator(line: &str) -> bool {
    line.strip_prefix("and ")
        .or_else(|| line.strip_prefix("or "))
        .or_else(|| line.strip_prefix("+ "))
        .is_some()
}

fn is_leading_string_literal(line: &str) -> bool {
    let mut prefix_len = 0;
    for ch in line.chars() {
        match ch {
            'b' | 'B' | 'f' | 'F' | 'r' | 'R' | 'u' | 'U' => prefix_len += ch.len_utf8(),
            '\'' | '"' => return true,
            _ => break,
        }
    }

    prefix_len > 0
        && line[prefix_len..]
            .chars()
            .next()
            .is_some_and(|ch| ch == '\'' || ch == '"')
}

fn last_significant_char(text: &str) -> Option<char> {
    text.chars().rev().find(|ch| !ch.is_whitespace())
}

fn delimiter_delta(line: &str) -> i32 {
    let mut delta = 0;
    let mut quote = None;
    let mut escaped = false;

    for ch in line.chars() {
        if let Some(quote_char) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_char {
                quote = None;
            }
            continue;
        }

        match ch {
            '#' => break,
            '\'' | '"' => quote = Some(ch),
            '(' | '[' | '{' => delta += 1,
            ')' | ']' | '}' => delta -= 1,
            _ => {}
        }
    }

    delta
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            ffi::py_finalize();
        }
    }
}
