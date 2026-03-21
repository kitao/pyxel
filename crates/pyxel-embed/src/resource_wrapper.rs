use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyResult, VirtualMachine};

use crate::helpers::*;

pub fn load(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel()
        .load_resource(name, None, None, None, None)
        .map_err(|e| vm.new_value_error(e))
}

pub fn load_pal(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel()
        .load_palette(name)
        .map_err(|e| vm.new_value_error(e))
}
