use std::process::{exit, Command};

use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

use crate::helpers::*;

fn register_reset_callback() {
    let exe = std::env::current_exe().unwrap_or_default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cwd = std::env::current_dir().unwrap_or_default();

    *pyxel::reset_callback() = Some(Box::new(move || {
        let _ = Command::new(&exe)
            .args(&args)
            .current_dir(&cwd)
            .envs(std::env::vars())
            .spawn();
        exit(0);
    }));
}

fn kw_str(args: &FuncArgs, name: &str) -> Option<String> {
    args.kwargs
        .get(name)
        .and_then(|o| s(o).map(|s| s.to_owned()))
}

fn kw_u32(args: &FuncArgs, name: &str, vm: &VirtualMachine) -> PyResult<Option<u32>> {
    match args.kwargs.get(name) {
        Some(o) => Ok(Some(u(o, vm)?)),
        None => Ok(None),
    }
}

fn kw_bool(args: &FuncArgs, name: &str) -> Option<bool> {
    args.kwargs.get(name).map(|o| ob_val(o))
}

fn ob_val(obj: &rustpython_vm::PyObjectRef) -> bool {
    use rustpython_vm::builtins::PyInt;
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v.as_bigint().try_into().unwrap_or(0);
        i != 0
    } else {
        false
    }
}

pub fn init(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let w = u(&a[0], vm)?;
    let h = u(&a[1], vm)?;

    // Title: positional arg[2] or keyword "title"
    let title_owned = a
        .get(2)
        .and_then(|o| {
            if vm.is_none(o) {
                None
            } else {
                s(o).map(|s| s.to_owned())
            }
        })
        .or_else(|| kw_str(&args, "title"));
    let title = title_owned.as_deref();

    let fps = kw_u32(&args, "fps", vm)?;
    let quit_key = kw_u32(&args, "quit_key", vm)?;
    let display_scale = kw_u32(&args, "display_scale", vm)?;
    let capture_scale = kw_u32(&args, "capture_scale", vm)?;
    let capture_sec = kw_u32(&args, "capture_sec", vm)?;
    let headless = kw_bool(&args, "headless");

    pyxel::init(
        w,
        h,
        title,
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
        headless,
    );
    register_reset_callback();
    Ok(())
}

pub fn run(update: PyObjectRef, draw: PyObjectRef, vm: &VirtualMachine) {
    struct Callback<'a> {
        vm: &'a VirtualMachine,
        update: PyObjectRef,
        draw: PyObjectRef,
    }

    impl pyxel::PyxelCallback for Callback<'_> {
        fn update(&mut self, _pyxel: &mut pyxel::Pyxel) {
            if let Err(exc) = self.vm.invoke(&self.update, ()) {
                self.vm.print_exception(exc);
                exit(1);
            }
        }

        fn draw(&mut self, _pyxel: &mut pyxel::Pyxel) {
            if let Err(exc) = self.vm.invoke(&self.draw, ()) {
                self.vm.print_exception(exc);
                exit(1);
            }
        }
    }

    pyxel::pyxel().run(Callback { vm, update, draw });
}

pub fn show() {
    pyxel::pyxel().show_screen();
}

pub fn flip() {
    pyxel::pyxel().flip_screen();
}

pub fn quit() {
    pyxel::pyxel().quit();
}

pub fn reset() {
    pyxel::pyxel().restart();
}

pub fn integer_scale(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let enabled = ob_val(&args.args[0]);
    pyxel::pyxel().set_integer_scale(enabled);
    Ok(())
}
