use std::ffi::CString;
use std::sync::{Mutex, MutexGuard};

use crate::{ffi, module};

static RUNTIME_LOCK: Mutex<()> = Mutex::new(());

const PYTHON_COMPAT_SOURCE: &str = r#"
from collections import deque as __pyxel_pocket_deque
import enum as __pyxel_pocket_enum
import math as __pyxel_pocket_math
import os as __pyxel_pocket_os
import pyxel as __pyxel_pocket_pyxel
import random as __pyxel_pocket_random

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

if not hasattr(__pyxel_pocket_enum, 'IntEnum'):
    __pyxel_pocket_enum.IntEnum = __pyxel_pocket_enum.Enum

    def __pyxel_pocket_intenum_value(value):
        if isinstance(value, __pyxel_pocket_enum.Enum):
            return value.value
        return value

    def __pyxel_pocket_enum_eq(self, other):
        if isinstance(other, __pyxel_pocket_enum.Enum):
            return type(self) == type(other) and self.value == other.value
        return self.value == other

    def __pyxel_pocket_enum_hash(self):
        return hash(self.value)

    def __pyxel_pocket_enum_str(self):
        if isinstance(self.value, int):
            return str(self.value)
        return type(self).__name__ + '.' + self.name

    def __pyxel_pocket_intenum_add(self, other):
        return self.value + __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_radd(self, other):
        return __pyxel_pocket_intenum_value(other) + self.value

    def __pyxel_pocket_intenum_sub(self, other):
        return self.value - __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_rsub(self, other):
        return __pyxel_pocket_intenum_value(other) - self.value

    def __pyxel_pocket_intenum_mod(self, other):
        return self.value % __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_lt(self, other):
        return self.value < __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_le(self, other):
        return self.value <= __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_gt(self, other):
        return self.value > __pyxel_pocket_intenum_value(other)

    def __pyxel_pocket_intenum_ge(self, other):
        return self.value >= __pyxel_pocket_intenum_value(other)

    __pyxel_pocket_enum.Enum.__eq__ = __pyxel_pocket_enum_eq
    __pyxel_pocket_enum.Enum.__hash__ = __pyxel_pocket_enum_hash
    __pyxel_pocket_enum.Enum.__str__ = __pyxel_pocket_enum_str
    __pyxel_pocket_enum.Enum.__add__ = __pyxel_pocket_intenum_add
    __pyxel_pocket_enum.Enum.__radd__ = __pyxel_pocket_intenum_radd
    __pyxel_pocket_enum.Enum.__sub__ = __pyxel_pocket_intenum_sub
    __pyxel_pocket_enum.Enum.__rsub__ = __pyxel_pocket_intenum_rsub
    __pyxel_pocket_enum.Enum.__mod__ = __pyxel_pocket_intenum_mod
    __pyxel_pocket_enum.Enum.__lt__ = __pyxel_pocket_intenum_lt
    __pyxel_pocket_enum.Enum.__le__ = __pyxel_pocket_intenum_le
    __pyxel_pocket_enum.Enum.__gt__ = __pyxel_pocket_intenum_gt
    __pyxel_pocket_enum.Enum.__ge__ = __pyxel_pocket_intenum_ge
    del __pyxel_pocket_enum_eq
    del __pyxel_pocket_enum_hash
    del __pyxel_pocket_enum_str
    del __pyxel_pocket_intenum_add
    del __pyxel_pocket_intenum_radd
    del __pyxel_pocket_intenum_sub
    del __pyxel_pocket_intenum_rsub
    del __pyxel_pocket_intenum_mod
    del __pyxel_pocket_intenum_lt
    del __pyxel_pocket_intenum_le
    del __pyxel_pocket_intenum_gt
    del __pyxel_pocket_intenum_ge

if not hasattr(__pyxel_pocket_enum, 'auto'):
    __pyxel_pocket_enum_auto_value = 0

    def __pyxel_pocket_enum_auto():
        global __pyxel_pocket_enum_auto_value
        __pyxel_pocket_enum_auto_value += 1
        return __pyxel_pocket_enum_auto_value

    __pyxel_pocket_enum.auto = __pyxel_pocket_enum_auto
    del __pyxel_pocket_enum_auto

def __pyxel_pocket_math_floor(x):
    value = int(x)
    if value > x:
        return value - 1
    return value

def __pyxel_pocket_math_ceil(x):
    value = int(x)
    if value < x:
        return value + 1
    return value

__pyxel_pocket_math.floor = __pyxel_pocket_math_floor
__pyxel_pocket_math.ceil = __pyxel_pocket_math_ceil
del __pyxel_pocket_math_floor
del __pyxel_pocket_math_ceil
del __pyxel_pocket_math

class __pyxel_pocket_Environ:
    def __getitem__(self, key):
        value = __pyxel_pocket_pyxel._get_env(str(key))
        if value is None:
            raise KeyError(key)
        return value

    def __setitem__(self, key, value):
        __pyxel_pocket_pyxel._set_env(str(key), str(value))

    def __delitem__(self, key):
        value = __pyxel_pocket_pyxel._get_env(str(key))
        if value is None:
            raise KeyError(key)
        __pyxel_pocket_pyxel._remove_env(str(key))

    def get(self, key, default=None):
        value = __pyxel_pocket_pyxel._get_env(str(key))
        if value is None:
            return default
        return value

    def pop(self, key, *defaults):
        value = __pyxel_pocket_pyxel._get_env(str(key))
        if value is None:
            if len(defaults) > 0:
                return defaults[0]
            raise KeyError(key)
        __pyxel_pocket_pyxel._remove_env(str(key))
        return value

    def __contains__(self, key):
        return __pyxel_pocket_pyxel._get_env(str(key)) is not None

__pyxel_pocket_os.environ = __pyxel_pocket_Environ()

def __pyxel_pocket_getenv(key, default=None):
    return __pyxel_pocket_os.environ.get(key, default)

__pyxel_pocket_os.getenv = __pyxel_pocket_getenv
del __pyxel_pocket_getenv
del __pyxel_pocket_Environ

__pyxel_pocket_random_N = 624
__pyxel_pocket_random_M = 397
__pyxel_pocket_random_MATRIX_A = 0x9908b0df
__pyxel_pocket_random_UPPER_MASK = 0x80000000
__pyxel_pocket_random_LOWER_MASK = 0x7fffffff
__pyxel_pocket_random_mt = [0] * __pyxel_pocket_random_N
__pyxel_pocket_random_index = __pyxel_pocket_random_N + 1

def __pyxel_pocket_random_u32(value):
    return value & 0xffffffff

def __pyxel_pocket_random_init_genrand(seed):
    global __pyxel_pocket_random_index
    __pyxel_pocket_random_mt[0] = __pyxel_pocket_random_u32(seed)
    i = 1
    while i < __pyxel_pocket_random_N:
        value = __pyxel_pocket_random_mt[i - 1]
        __pyxel_pocket_random_mt[i] = __pyxel_pocket_random_u32(
            1812433253 * (value ^ (value >> 30)) + i
        )
        i += 1
    __pyxel_pocket_random_index = __pyxel_pocket_random_N

def __pyxel_pocket_random_init_by_array(keys):
    global __pyxel_pocket_random_index
    __pyxel_pocket_random_init_genrand(19650218)
    i = 1
    j = 0
    k = __pyxel_pocket_random_N
    if len(keys) > k:
        k = len(keys)
    while k > 0:
        value = __pyxel_pocket_random_mt[i - 1]
        mixed = __pyxel_pocket_random_mt[i] ^ (
            (value ^ (value >> 30)) * 1664525
        )
        __pyxel_pocket_random_mt[i] = __pyxel_pocket_random_u32(mixed + keys[j] + j)
        i += 1
        j += 1
        if i >= __pyxel_pocket_random_N:
            __pyxel_pocket_random_mt[0] = __pyxel_pocket_random_mt[__pyxel_pocket_random_N - 1]
            i = 1
        if j >= len(keys):
            j = 0
        k -= 1
    k = __pyxel_pocket_random_N - 1
    while k > 0:
        value = __pyxel_pocket_random_mt[i - 1]
        mixed = __pyxel_pocket_random_mt[i] ^ (
            (value ^ (value >> 30)) * 1566083941
        )
        __pyxel_pocket_random_mt[i] = __pyxel_pocket_random_u32(mixed - i)
        i += 1
        if i >= __pyxel_pocket_random_N:
            __pyxel_pocket_random_mt[0] = __pyxel_pocket_random_mt[__pyxel_pocket_random_N - 1]
            i = 1
        k -= 1
    __pyxel_pocket_random_mt[0] = 0x80000000
    __pyxel_pocket_random_index = __pyxel_pocket_random_N

def __pyxel_pocket_random_seed(seed=None):
    if seed is None:
        seed = 5489
    if seed < 0:
        seed = -seed
    keys = []
    while seed > 0:
        keys.append(seed & 0xffffffff)
        seed = seed >> 32
    if len(keys) == 0:
        keys.append(0)
    __pyxel_pocket_random_init_by_array(keys)

def __pyxel_pocket_random_genrand_u32():
    global __pyxel_pocket_random_index
    mag01 = [0, __pyxel_pocket_random_MATRIX_A]
    if __pyxel_pocket_random_index >= __pyxel_pocket_random_N:
        kk = 0
        while kk < __pyxel_pocket_random_N - __pyxel_pocket_random_M:
            y = (__pyxel_pocket_random_mt[kk] & __pyxel_pocket_random_UPPER_MASK) | (__pyxel_pocket_random_mt[kk + 1] & __pyxel_pocket_random_LOWER_MASK)
            mixed = __pyxel_pocket_random_mt[kk + __pyxel_pocket_random_M] ^ (y >> 1) ^ mag01[y & 1]
            __pyxel_pocket_random_mt[kk] = __pyxel_pocket_random_u32(mixed)
            kk += 1
        while kk < __pyxel_pocket_random_N - 1:
            y = (__pyxel_pocket_random_mt[kk] & __pyxel_pocket_random_UPPER_MASK) | (__pyxel_pocket_random_mt[kk + 1] & __pyxel_pocket_random_LOWER_MASK)
            mixed = __pyxel_pocket_random_mt[kk + (__pyxel_pocket_random_M - __pyxel_pocket_random_N)] ^ (y >> 1) ^ mag01[y & 1]
            __pyxel_pocket_random_mt[kk] = __pyxel_pocket_random_u32(mixed)
            kk += 1
        y = (__pyxel_pocket_random_mt[__pyxel_pocket_random_N - 1] & __pyxel_pocket_random_UPPER_MASK) | (__pyxel_pocket_random_mt[0] & __pyxel_pocket_random_LOWER_MASK)
        mixed = __pyxel_pocket_random_mt[__pyxel_pocket_random_M - 1] ^ (y >> 1) ^ mag01[y & 1]
        __pyxel_pocket_random_mt[__pyxel_pocket_random_N - 1] = __pyxel_pocket_random_u32(mixed)
        __pyxel_pocket_random_index = 0
    y = __pyxel_pocket_random_mt[__pyxel_pocket_random_index]
    __pyxel_pocket_random_index += 1
    y = y ^ (y >> 11)
    y = y ^ ((y << 7) & 0x9d2c5680)
    y = y ^ ((y << 15) & 0xefc60000)
    y = y ^ (y >> 18)
    return __pyxel_pocket_random_u32(y)

def __pyxel_pocket_random_random():
    a = __pyxel_pocket_random_genrand_u32() >> 5
    b = __pyxel_pocket_random_genrand_u32() >> 6
    return (a * 67108864.0 + b) / 9007199254740992.0

def __pyxel_pocket_random_getrandbits(k):
    result = 0
    while k >= 32:
        result = (result << 32) | __pyxel_pocket_random_genrand_u32()
        k -= 32
    if k > 0:
        result = (result << k) | (__pyxel_pocket_random_genrand_u32() >> (32 - k))
    return result

def __pyxel_pocket_random_randbelow(n):
    if n <= 0:
        raise ValueError('n must be greater than 0')
    k = 0
    value = n
    while value > 0:
        k += 1
        value = value >> 1
    r = __pyxel_pocket_random_getrandbits(k)
    while r >= n:
        r = __pyxel_pocket_random_getrandbits(k)
    return r

def __pyxel_pocket_random_randrange(start, stop=None, step=1):
    if stop is None:
        stop = start
        start = 0
    width = stop - start
    if step == 1:
        return start + __pyxel_pocket_random_randbelow(width)
    count = (width + step - 1) // step
    return start + step * __pyxel_pocket_random_randbelow(count)

def __pyxel_pocket_random_randint(a, b):
    return a + __pyxel_pocket_random_randbelow(b - a + 1)

def __pyxel_pocket_random_uniform(a, b):
    return a + (b - a) * __pyxel_pocket_random_random()

def __pyxel_pocket_random_choice(seq):
    return seq[__pyxel_pocket_random_randbelow(len(seq))]

def __pyxel_pocket_random_shuffle(seq):
    i = len(seq) - 1
    while i > 0:
        j = __pyxel_pocket_random_randbelow(i + 1)
        tmp = seq[i]
        seq[i] = seq[j]
        seq[j] = tmp
        i -= 1

def __pyxel_pocket_random_sample(population, k):
    pool = list(population)
    if k < 0 or k > len(pool):
        raise ValueError('Sample larger than population or is negative')
    n = len(pool)
    result = []
    setsize = 21
    if k > 5:
        size = k * 3
        power = 1
        while power < size:
            power *= 4
        setsize += power
    if n <= setsize:
        for i in range(k):
            index = __pyxel_pocket_random_randbelow(n - i)
            result.append(pool[index])
            pool[index] = pool[n - i - 1]
    else:
        selected = []
        for i in range(k):
            index = __pyxel_pocket_random_randbelow(n)
            while index in selected:
                index = __pyxel_pocket_random_randbelow(n)
            selected.append(index)
            result.append(pool[index])
    return result

__pyxel_pocket_random.seed = __pyxel_pocket_random_seed
__pyxel_pocket_random.random = __pyxel_pocket_random_random
__pyxel_pocket_random.randrange = __pyxel_pocket_random_randrange
__pyxel_pocket_random.randint = __pyxel_pocket_random_randint
__pyxel_pocket_random.uniform = __pyxel_pocket_random_uniform
__pyxel_pocket_random.choice = __pyxel_pocket_random_choice
__pyxel_pocket_random.shuffle = __pyxel_pocket_random_shuffle
__pyxel_pocket_random.sample = __pyxel_pocket_random_sample
__pyxel_pocket_random_seed()
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

const ITERTOOLS_COMPAT_SOURCE: &str = r"
def filterfalse(predicate, iterable):
    if predicate is None:
        predicate = bool
    return [item for item in iterable if not predicate(item)]
";

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
        install_itertools_compat();
        Self { _guard: guard }
    }

    pub fn exec_source(&self, source: &str, filename: &str) -> Result<(), String> {
        exec_source_in_current_runtime(source, filename)
    }
}

pub(crate) fn exec_source_in_current_runtime(source: &str, filename: &str) -> Result<(), String> {
    let source = normalize_source(source);
    let source = CString::new(source).map_err(|_| "source contains NUL byte".to_owned())?;
    let filename = CString::new(filename).map_err(|_| "filename contains NUL byte".to_owned())?;
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

fn install_itertools_compat() {
    let source = CString::new(ITERTOOLS_COMPAT_SOURCE).expect("itertools source contains NUL byte");
    let filename = CString::new("<pyxel-pocket-itertools>").unwrap();
    let module = unsafe { ffi::py_newmodule(c"itertools".as_ptr()) };
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
        panic!("failed to install PocketPy itertools compatibility module");
    }
}

pub(crate) fn normalize_source(source: &str) -> String {
    let mut normalized = String::with_capacity(source.len());
    let mut open_delimiters = 0i32;
    let mut unpack_index = 0usize;
    let mut slice_assign_index = 0usize;
    let mut any_index = 0usize;
    let lines = source.lines().collect::<Vec<_>>();
    let mut line_index = 0usize;

    while line_index < lines.len() {
        let raw_line = lines[line_index];
        if raw_line.trim() == "import pyxel.cli" {
            let indent_len = raw_line.len() - raw_line.trim_start().len();
            normalized.push_str(&raw_line[..indent_len]);
            normalized.push_str("import pyxel\n");
            line_index += 1;
            continue;
        }

        if let Some((expanded, next_index)) = parenthesized_from_import(&lines, line_index) {
            normalized.push_str(&expanded);
            line_index = next_index;
            continue;
        }

        if let Some(expanded) = named_default_arguments(&lines, line_index) {
            normalized.push_str(&expanded);
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

        let normalized_line = normalize_unary_plus(raw_line);
        let line = normalized_line.as_str();

        if let Some(expanded) = one_line_generator_call(line) {
            normalized.push_str(&expanded);
            normalized.push('\n');
            line_index += 1;
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
        let joins_after_trailing_operator = open_delimiters > 0 && !normalized.ends_with('\n');
        let joins_leading_operator = open_delimiters > 0 && is_leading_operator(trimmed);
        let joins_adjacent_string = open_delimiters > 0
            && is_leading_string_literal(trimmed)
            && matches!(last_significant_char(&normalized), Some('\'' | '"'));
        let continues_trailing_join = open_delimiters > 0
            && (is_trailing_operator(trimmed) || is_trailing_value_colon(trimmed));

        if joins_after_trailing_operator || joins_leading_operator || joins_adjacent_string {
            if normalized.ends_with('\n') {
                normalized.pop();
            }
            if joins_adjacent_string {
                normalized.push_str(" + ");
            } else if !joins_after_trailing_operator {
                normalized.push(' ');
            }
            normalized.push_str(trimmed);
        } else {
            normalized.push_str(line);
        }
        if continues_trailing_join {
            normalized.push(' ');
        } else {
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

fn parenthesized_from_import(lines: &[&str], index: usize) -> Option<(String, usize)> {
    let line = *lines.get(index)?;
    let trimmed = line.trim_start();
    let prefix = trimmed.strip_suffix(" import (")?;
    if !prefix.starts_with("from ") {
        return None;
    }

    let indent = &line[..line.len() - trimmed.len()];
    let mut names = Vec::new();
    let mut next_index = index + 1;
    while next_index < lines.len() {
        let name = lines[next_index].trim();
        if name == ")" {
            let mut expanded = String::new();
            expanded.push_str(indent);
            expanded.push_str(prefix);
            expanded.push_str(" import ");
            expanded.push_str(&names.join(", "));
            expanded.push('\n');
            return Some((expanded, next_index + 1));
        }
        names.push(name.trim_end_matches(',').to_owned());
        next_index += 1;
    }
    None
}

fn named_default_arguments(lines: &[&str], index: usize) -> Option<String> {
    let line = *lines.get(index)?;
    let trimmed = line.trim_start();
    if !trimmed.starts_with("def ") || !trimmed.ends_with("):") {
        return None;
    }

    let open = trimmed.find('(')?;
    let close = trimmed.rfind("):")?;
    let params = &trimmed[open + 1..close];
    let mut new_params = Vec::new();
    let mut defaults = Vec::new();
    for param in split_top_level_commas(params) {
        let param_trimmed = param.trim();
        if let Some((name, default_value)) = param_trimmed.split_once('=') {
            let name = name.trim();
            let default_value = default_value.trim();
            if needs_runtime_default(default_value) {
                new_params.push(format!("{name}=None"));
                defaults.push((name.to_owned(), default_value.to_owned()));
                continue;
            }
        }
        new_params.push(param_trimmed.to_owned());
    }
    if defaults.is_empty() {
        return None;
    }

    let indent = &line[..line.len() - trimmed.len()];
    let body_indent = next_body_indent(lines, index).unwrap_or_else(|| format!("{indent}    "));
    let mut expanded = String::new();
    expanded.push_str(indent);
    expanded.push_str(&trimmed[..=open]);
    expanded.push_str(&new_params.join(", "));
    expanded.push_str("):\n");
    for (name, default_value) in defaults {
        expanded.push_str(&body_indent);
        expanded.push_str("if ");
        expanded.push_str(&name);
        expanded.push_str(" is None:\n");
        expanded.push_str(&body_indent);
        expanded.push_str("    ");
        expanded.push_str(&name);
        expanded.push_str(" = ");
        expanded.push_str(&default_value);
        expanded.push('\n');
    }
    Some(expanded)
}

fn needs_runtime_default(default_value: &str) -> bool {
    if matches!(default_value, "None" | "True" | "False") {
        return false;
    }
    default_value
        .chars()
        .all(|ch| ch == '_' || ch == '.' || ch.is_ascii_alphanumeric())
        && default_value
            .chars()
            .next()
            .is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic())
}

fn split_top_level_commas(value: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in value.char_indices() {
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
            '\'' | '"' => quote = Some(ch),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&value[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(&value[start..]);
    parts
}

fn next_body_indent(lines: &[&str], index: usize) -> Option<String> {
    for line in lines.iter().skip(index + 1) {
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        return Some(line[..line.len() - trimmed.len()].to_owned());
    }
    None
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

fn one_line_generator_call(line: &str) -> Option<String> {
    for function in ["sum", "any", "all", "min", "max", "enumerate"] {
        let needle = format!("{function}(");
        let Some(start) = line.find(&needle) else {
            continue;
        };
        if start > 0 && line[..start].chars().last().is_some_and(is_identifier_char) {
            continue;
        }

        let body_start = start + needle.len();
        let body_end = body_start + line[body_start..].rfind(')')?;
        let body = &line[body_start..body_end];
        if !body.contains(" for ") || body.trim_start().starts_with('[') {
            continue;
        }

        let mut expanded = String::new();
        expanded.push_str(&line[..body_start]);
        expanded.push('[');
        expanded.push_str(body);
        expanded.push_str("])");
        expanded.push_str(&line[body_end + 1..]);
        return Some(expanded);
    }
    None
}

fn normalize_unary_plus(line: &str) -> String {
    let mut normalized = line.to_owned();
    for (from, to) in [
        ("assert +", "assert "),
        ("return +", "return "),
        ("(+", "("),
        ("[+", "["),
        ("{+", "{"),
        (", +", ", "),
        ("= +", "= "),
        (": +", ": "),
    ] {
        normalized = normalized.replace(from, to);
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
    ["and ", "or ", "+ ", "== ", "!= ", "<= ", ">= ", "< ", "> "]
        .iter()
        .any(|operator| line.starts_with(operator))
}

fn is_trailing_operator(line: &str) -> bool {
    let line = line.trim_end();
    [" and", " or", " +", " ==", " !=", " <=", " >=", " <", " >"]
        .iter()
        .any(|operator| line.ends_with(operator))
}

fn is_trailing_value_colon(line: &str) -> bool {
    let line = line.trim_end();
    line.ends_with(':') && !line.ends_with("):")
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

fn is_identifier_char(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
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
        crate::runner::clear_extracted_apps();
        unsafe {
            ffi::py_finalize();
        }
    }
}
