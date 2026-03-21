use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyResult, VirtualMachine};

use crate::helpers::*;

pub fn btn(args: FuncArgs, vm: &VirtualMachine) -> PyResult<bool> {
    Ok(pyxel::pyxel().is_button_down(u(&args.args[0], vm)?))
}

pub fn btnp(args: FuncArgs, vm: &VirtualMachine) -> PyResult<bool> {
    let a = &args.args;
    Ok(pyxel::pyxel().is_button_pressed(u(&a[0], vm)?, ou(a, 1, vm)?, ou(a, 2, vm)?))
}

pub fn btnr(args: FuncArgs, vm: &VirtualMachine) -> PyResult<bool> {
    Ok(pyxel::pyxel().is_button_released(u(&args.args[0], vm)?))
}

pub fn btnv(args: FuncArgs, vm: &VirtualMachine) -> PyResult<i32> {
    Ok(pyxel::pyxel().button_value(u(&args.args[0], vm)?))
}

pub fn mouse(visible: bool) {
    pyxel::pyxel().set_mouse_visible(visible);
}
