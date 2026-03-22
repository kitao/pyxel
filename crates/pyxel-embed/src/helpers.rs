// Type conversion helpers for RustPython → pyxel-core

use rustpython_vm as vm;
use vm::builtins::{PyFloat, PyInt};

pub fn f(obj: &vm::PyObjectRef, vm: &vm::VirtualMachine) -> vm::PyResult<f32> {
    if let Some(v) = obj.payload::<PyFloat>() {
        return Ok(v.to_f64() as f32);
    }
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v
            .as_bigint()
            .try_into()
            .map_err(|_| vm.new_overflow_error("int too large".into()))?;
        return Ok(i as f32);
    }
    Err(vm.new_type_error("expected number".into()))
}

pub fn u(obj: &vm::PyObjectRef, vm: &vm::VirtualMachine) -> vm::PyResult<u32> {
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v
            .as_bigint()
            .try_into()
            .map_err(|_| vm.new_overflow_error("int too large".into()))?;
        return Ok(i as u32);
    }
    Err(vm.new_type_error("expected int".into()))
}

pub fn c(obj: &vm::PyObjectRef, vm: &vm::VirtualMachine) -> vm::PyResult<u8> {
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v
            .as_bigint()
            .try_into()
            .map_err(|_| vm.new_overflow_error("int too large".into()))?;
        return Ok(i as u8);
    }
    Err(vm.new_type_error("expected int".into()))
}

pub fn s(obj: &vm::PyObjectRef) -> Option<&str> {
    obj.payload::<vm::builtins::PyStr>().map(|s| s.as_str())
}

pub fn of(a: &[vm::PyObjectRef], i: usize, vm: &vm::VirtualMachine) -> vm::PyResult<Option<f32>> {
    match a.get(i) {
        Some(o) if !vm.is_none(o) => Ok(Some(f(o, vm)?)),
        _ => Ok(None),
    }
}

pub fn oc(a: &[vm::PyObjectRef], i: usize, vm: &vm::VirtualMachine) -> vm::PyResult<Option<u8>> {
    match a.get(i) {
        Some(o) if !vm.is_none(o) => Ok(Some(c(o, vm)?)),
        _ => Ok(None),
    }
}

pub fn ou(a: &[vm::PyObjectRef], i: usize, vm: &vm::VirtualMachine) -> vm::PyResult<Option<u32>> {
    match a.get(i) {
        Some(o) if !vm.is_none(o) => Ok(Some(u(o, vm)?)),
        _ => Ok(None),
    }
}

pub fn ob(a: &[vm::PyObjectRef], i: usize, vm: &vm::VirtualMachine) -> Option<bool> {
    match a.get(i) {
        Some(o) if !vm.is_none(o) => {
            if let Some(v) = o.payload::<PyInt>() {
                let i: i64 = v.as_bigint().try_into().unwrap_or(0);
                Some(i != 0)
            } else {
                Some(false)
            }
        }
        _ => None,
    }
}

pub fn i(obj: &vm::PyObjectRef, vm: &vm::VirtualMachine) -> vm::PyResult<i32> {
    if let Some(v) = obj.payload::<PyInt>() {
        let val: i64 = v
            .as_bigint()
            .try_into()
            .map_err(|_| vm.new_overflow_error("int too large".into()))?;
        return Ok(val as i32);
    }
    Err(vm.new_type_error("expected int".into()))
}

pub fn i64_val(obj: &vm::PyObjectRef, vm: &vm::VirtualMachine) -> vm::PyResult<i64> {
    if let Some(v) = obj.payload::<PyInt>() {
        let val: i64 = v
            .as_bigint()
            .try_into()
            .map_err(|_| vm.new_overflow_error("int too large".into()))?;
        return Ok(val);
    }
    Err(vm.new_type_error("expected int".into()))
}

pub fn to_bool(obj: &vm::PyObjectRef) -> bool {
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v.as_bigint().try_into().unwrap_or(0);
        return i != 0;
    }
    false
}

pub fn kw_str(args: &vm::function::FuncArgs, name: &str) -> Option<String> {
    args.kwargs
        .get(name)
        .and_then(|o| s(o).map(|s| s.to_owned()))
}

pub fn kw_u32(
    args: &vm::function::FuncArgs,
    name: &str,
    vm: &vm::VirtualMachine,
) -> vm::PyResult<Option<u32>> {
    match args.kwargs.get(name) {
        Some(o) => Ok(Some(u(o, vm)?)),
        None => Ok(None),
    }
}

pub fn kw_bool(args: &vm::function::FuncArgs, name: &str) -> Option<bool> {
    args.kwargs.get(name).map(to_bool)
}

pub fn kw_f32(
    args: &vm::function::FuncArgs,
    name: &str,
    vm: &vm::VirtualMachine,
) -> vm::PyResult<Option<f32>> {
    match args.kwargs.get(name) {
        Some(o) if !vm.is_none(o) => Ok(Some(f(o, vm)?)),
        _ => Ok(None),
    }
}
