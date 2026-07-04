use std::ffi::CString;
use std::sync::{Mutex, MutexGuard};

use crate::{ffi, module};

static RUNTIME_LOCK: Mutex<()> = Mutex::new(());

const PYTHON_COMPAT_SOURCE: &str = r#"
from collections import deque as __pyxel_pocket_deque
import os as __pyxel_pocket_os

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

def __pyxel_pocket_dict_iter(self):
    return iter(self.keys())

dict.__iter__ = __pyxel_pocket_dict_iter
del __pyxel_pocket_dict_iter

def __pyxel_pocket_str_capitalize(self):
    if len(self) == 0:
        return self
    return self[0].upper() + self[1:].lower()

str.capitalize = __pyxel_pocket_str_capitalize
del __pyxel_pocket_str_capitalize

if not hasattr(__pyxel_pocket_os, 'environ'):
    __pyxel_pocket_os.environ = {}
del __pyxel_pocket_os
"#;

const PATHLIB_COMPAT_SOURCE: &str = r#"
import os

def __pyxel_pocket_norm(path):
    parts = []
    is_abs = path.startswith('/')
    for part in path.split('/'):
        if part == '' or part == '.':
            continue
        if part == '..':
            if parts:
                parts.pop()
            continue
        parts.append(part)
    result = '/'.join(parts)
    if is_abs:
        result = '/' + result
    return result or ('/' if is_abs else '.')

def __pyxel_pocket_join(left, right):
    right = str(right)
    if right.startswith('/'):
        return __pyxel_pocket_norm(right)
    return __pyxel_pocket_norm(str(left).rstrip('/') + '/' + right)

def __pyxel_pocket_parent(path):
    path = __pyxel_pocket_norm(str(path))
    if path == '/':
        return '/'
    if '/' not in path:
        return '.'
    parts = path.split('/')
    parts.pop()
    result = '/'.join(parts)
    return result or ('/' if path.startswith('/') else '.')

class Path:
    def __init__(self, *parts):
        if not parts:
            path = '.'
        else:
            path = str(parts[0])
            for part in parts[1:]:
                path = __pyxel_pocket_join(path, part)
        self._path = __pyxel_pocket_norm(path)

    @property
    def parent(self):
        return Path(__pyxel_pocket_parent(self._path))

    def resolve(self):
        if self._path.startswith('/'):
            return Path(self._path)
        return Path(__pyxel_pocket_join(os.getcwd(), self._path))

    def __truediv__(self, other):
        return Path(__pyxel_pocket_join(self._path, other))

    def __str__(self):
        return self._path

    def __repr__(self):
        return "Path('" + self._path + "')"
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
        install_pathlib_compat();
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

fn install_pathlib_compat() {
    let source = CString::new(PATHLIB_COMPAT_SOURCE).expect("pathlib source contains NUL byte");
    let filename = CString::new("<pyxel-pocket-pathlib>").unwrap();
    let module = unsafe { ffi::py_newmodule(c"pathlib".as_ptr()) };
    let ok = unsafe {
        ffi::py_exec(
            source.as_ptr(),
            filename.as_ptr(),
            ffi::py_CompileMode_EXEC_MODE,
            module,
        )
    };
    if !ok {
        unsafe {
            ffi::py_printexc();
        }
        panic!("failed to install PocketPy pathlib compatibility module");
    }
}

fn normalize_source(source: &str) -> String {
    let mut normalized = String::with_capacity(source.len());
    let mut open_delimiters = 0i32;
    let mut unpack_index = 0usize;
    let mut slice_assign_index = 0usize;
    let mut any_index = 0usize;
    let lines = source.lines().collect::<Vec<_>>();
    let mut line_index = 0usize;

    while line_index < lines.len() {
        let line = lines[line_index];
        if line.trim() == "import pyxel.cli" {
            let indent_len = line.len() - line.trim_start().len();
            normalized.push_str(&line[..indent_len]);
            normalized.push_str("import pyxel\n");
            line_index += 1;
            continue;
        }

        if let Some((expanded, next_index)) = multiline_any_generator(&lines, line_index, any_index)
        {
            normalized.push_str(&expanded);
            any_index += 1;
            line_index = next_index;
            continue;
        }

        if let Some((expanded, next_index)) = multiline_enumerate_generator(&lines, line_index) {
            normalized.push_str(&expanded);
            line_index = next_index;
            continue;
        }

        if let Some(expanded) = nested_list_comprehension_assignment(line) {
            normalized.push_str(&expanded);
            line_index += 1;
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
            line_index += 1;
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
            line_index += 1;
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
        line_index += 1;
    }

    if !source.ends_with('\n') && normalized.ends_with('\n') {
        normalized.pop();
    }
    normalized
}

fn multiline_any_generator(
    lines: &[&str],
    index: usize,
    any_index: usize,
) -> Option<(String, usize)> {
    let line = *lines.get(index)?;
    let trimmed = line.trim_start();
    if trimmed != "if any(" {
        return None;
    }

    let indent = &line[..line.len() - trimmed.len()];
    let condition = lines.get(index + 1)?.trim();
    let for_line = lines.get(index + 2)?.trim();
    if !for_line.starts_with("for ") || !for_line.contains(" in ") {
        return None;
    }

    let mut end_index = index + 3;
    while end_index < lines.len() && lines[end_index].trim() != "):" {
        end_index += 1;
    }
    if end_index >= lines.len() {
        return None;
    }

    let temp = format!("__pyxel_pocket_any_{any_index}");
    let mut expanded = String::new();
    expanded.push_str(indent);
    expanded.push_str(&temp);
    expanded.push_str(" = False\n");
    expanded.push_str(indent);
    expanded.push_str(for_line);

    if end_index == index + 3 {
        expanded.push_str(":\n");
    } else {
        expanded.push('\n');
        for iterable_line in &lines[index + 3..end_index] {
            expanded.push_str(iterable_line);
            if iterable_line.trim() == "]" || iterable_line.trim() == ")" {
                expanded.push(':');
            }
            expanded.push('\n');
        }
    }

    expanded.push_str(indent);
    expanded.push_str("    if ");
    expanded.push_str(condition);
    expanded.push_str(":\n");
    expanded.push_str(indent);
    expanded.push_str("        ");
    expanded.push_str(&temp);
    expanded.push_str(" = True\n");
    expanded.push_str(indent);
    expanded.push_str("        break\n");
    expanded.push_str(indent);
    expanded.push_str("if ");
    expanded.push_str(&temp);
    expanded.push_str(":\n");

    Some((expanded, end_index + 1))
}

fn multiline_enumerate_generator(lines: &[&str], index: usize) -> Option<(String, usize)> {
    let line = *lines.get(index)?;
    let trimmed = line.trim_start();
    if !trimmed.starts_with("for ") || !trimmed.ends_with(" in enumerate(") {
        return None;
    }

    let generator = lines.get(index + 1)?.trim();
    if !generator.contains(" for ") {
        return None;
    }
    if lines.get(index + 2)?.trim() != "):" {
        return None;
    }

    let indent = &line[..line.len() - trimmed.len()];
    let mut expanded = String::new();
    expanded.push_str(indent);
    expanded.push_str(trimmed);
    expanded.push('[');
    expanded.push_str(generator);
    expanded.push_str("]):\n");
    Some((expanded, index + 3))
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
