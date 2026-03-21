use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyResult, VirtualMachine};

use crate::helpers::*;

pub fn ceil(args: FuncArgs, vm: &VirtualMachine) -> PyResult<i32> {
    Ok(pyxel::Pyxel::ceil(f(&args.args[0], vm)?))
}

pub fn floor(args: FuncArgs, vm: &VirtualMachine) -> PyResult<i32> {
    Ok(pyxel::Pyxel::floor(f(&args.args[0], vm)?))
}

pub fn sqrt(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    Ok(pyxel::Pyxel::sqrt(f(&args.args[0], vm)?) as f64)
}

pub fn sin(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    Ok(pyxel::Pyxel::sin(f(&args.args[0], vm)?) as f64)
}

pub fn cos(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    Ok(pyxel::Pyxel::cos(f(&args.args[0], vm)?) as f64)
}

pub fn atan2(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    let a = &args.args;
    Ok(pyxel::Pyxel::atan2(f(&a[0], vm)?, f(&a[1], vm)?) as f64)
}

pub fn rseed(seed: u32) {
    pyxel::Pyxel::random_seed(seed);
}

pub fn rndi(a: i32, b: i32) -> i32 {
    pyxel::Pyxel::random_int(a, b)
}

pub fn rndf(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    let a = &args.args;
    Ok(pyxel::Pyxel::random_float(f(&a[0], vm)?, f(&a[1], vm)?) as f64)
}

pub fn nseed(seed: u32) {
    pyxel::Pyxel::noise_seed(seed);
}

pub fn noise(args: FuncArgs, vm: &VirtualMachine) -> PyResult<f64> {
    let a = &args.args;
    let x = f(&a[0], vm)?;
    let y = if a.len() > 1 { f(&a[1], vm)? } else { 0.0 };
    let z = if a.len() > 2 { f(&a[2], vm)? } else { 0.0 };
    Ok(pyxel::Pyxel::noise(x, y, z) as f64)
}

pub fn clamp(args: FuncArgs, vm: &VirtualMachine) -> PyResult<rustpython_vm::PyObjectRef> {
    let a = &args.args;
    // Try integer first, then float
    if let (Ok(x), Ok(lo), Ok(hi)) = (u(&a[0], vm), u(&a[1], vm), u(&a[2], vm)) {
        let x = x as i64;
        let lo = lo as i64;
        let hi = hi as i64;
        let (lo, hi) = if lo < hi { (lo, hi) } else { (hi, lo) };
        let v = x.max(lo).min(hi);
        return Ok(vm.new_pyobj(v));
    }
    let x = f(&a[0], vm)?;
    let lo = f(&a[1], vm)?;
    let hi = f(&a[2], vm)?;
    let (lo, hi) = if lo < hi { (lo, hi) } else { (hi, lo) };
    let v = x.max(lo).min(hi);
    Ok(vm.new_pyobj(v as f64))
}
