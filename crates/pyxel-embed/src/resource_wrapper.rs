use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyResult, VirtualMachine};

use crate::helpers::*;

pub fn load(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    let exclude_images = kw_bool(&args, "exclude_images");
    let exclude_tilemaps = kw_bool(&args, "exclude_tilemaps");
    let exclude_sounds = kw_bool(&args, "exclude_sounds");
    let exclude_musics = kw_bool(&args, "exclude_musics");
    pyxel::pyxel()
        .load_resource(
            name,
            exclude_images,
            exclude_tilemaps,
            exclude_sounds,
            exclude_musics,
        )
        .map_err(|e| vm.new_value_error(e))
}

pub fn save(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    let exclude_images = kw_bool(&args, "exclude_images");
    let exclude_tilemaps = kw_bool(&args, "exclude_tilemaps");
    let exclude_sounds = kw_bool(&args, "exclude_sounds");
    let exclude_musics = kw_bool(&args, "exclude_musics");
    pyxel::pyxel()
        .save_resource(
            name,
            exclude_images,
            exclude_tilemaps,
            exclude_sounds,
            exclude_musics,
        )
        .map_err(|e| vm.new_value_error(e))
}

pub fn load_pal(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel()
        .load_palette(name)
        .map_err(|e| vm.new_value_error(e))
}

pub fn save_pal(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let name = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel()
        .save_palette(name)
        .map_err(|e| vm.new_value_error(e))
}

pub fn screenshot(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let scale = if args.args.is_empty() {
        None
    } else {
        ou(&args.args, 0, vm)?
    };
    pyxel::pyxel()
        .take_screenshot(scale)
        .map_err(|e| vm.new_value_error(e))
}

pub fn screencast(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let scale = if args.args.is_empty() {
        None
    } else {
        ou(&args.args, 0, vm)?
    };
    pyxel::pyxel()
        .save_screencast(scale)
        .map_err(|e| vm.new_value_error(e))
}

pub fn reset_screencast() {
    pyxel::pyxel().reset_screencast();
}

pub fn user_data_dir(args: FuncArgs, vm: &VirtualMachine) -> PyResult<String> {
    let a = &args.args;
    let vendor = s(&a[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    let app = s(&a[1]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel()
        .user_data_dir(vendor, app)
        .map_err(|e| vm.new_value_error(e))
}
