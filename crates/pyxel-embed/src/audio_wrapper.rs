use rustpython_vm::builtins::{PyInt, PyList};
use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

use crate::helpers::*;

// Extract bool from a PyObjectRef (for kwargs)
fn to_bool(obj: &PyObjectRef) -> bool {
    if let Some(v) = obj.payload::<PyInt>() {
        let i: i64 = v.as_bigint().try_into().unwrap_or(0);
        return i != 0;
    }
    false
}

// Extract Vec<u32> from a Python list of ints
fn to_u32_list(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Vec<u32>> {
    let list = obj
        .payload::<PyList>()
        .ok_or_else(|| vm.new_type_error("expected list of int".into()))?;
    let items = list.borrow_vec();
    items.iter().map(|item| u(item, vm)).collect()
}

pub fn play(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let ch = u(&a[0], vm)?;
    let snd_obj = &a[1];
    let loop_ = args
        .kwargs
        .get("loop")
        .map(|o| to_bool(o))
        .or_else(|| ob(a, 2, vm))
        .unwrap_or(false);
    let resume = args
        .kwargs
        .get("resume")
        .map(|o| to_bool(o))
        .or_else(|| ob(a, 3, vm))
        .unwrap_or(false);

    // snd: int, list of int, or str
    if let Ok(snd) = u(snd_obj, vm) {
        pyxel::pyxel().play_sound(ch, snd, None, loop_, resume);
    } else if let Ok(snd_list) = to_u32_list(snd_obj, vm) {
        pyxel::pyxel().play(ch, &snd_list, None, loop_, resume);
    } else if let Some(code) = s(snd_obj) {
        pyxel::pyxel()
            .play_mml(ch, code, None, loop_, resume)
            .map_err(|e| vm.new_value_error(e))?;
    } else {
        return Err(vm.new_type_error("expected int, list, or str for snd".into()));
    }
    Ok(())
}

pub fn playm(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let msc = u(&a[0], vm)?;
    let loop_ = args
        .kwargs
        .get("loop")
        .map(|o| to_bool(o))
        .or_else(|| ob(a, 1, vm))
        .unwrap_or(false);
    pyxel::pyxel().play_music(msc, None, loop_);
    Ok(())
}

pub fn stop(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    if a.is_empty() {
        pyxel::pyxel().stop_all_channels();
    } else {
        pyxel::pyxel().stop_channel(u(&a[0], vm)?);
    }
    Ok(())
}

pub fn play_pos(args: FuncArgs, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
    let ch = u(&args.args[0], vm)?;
    match pyxel::pyxel().play_position(ch) {
        Some((snd, pos)) => Ok(vm.new_pyobj((vm.new_pyobj(snd), vm.new_pyobj(pos)))),
        None => Ok(vm.ctx.none()),
    }
}

pub fn gen_bgm(args: FuncArgs, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
    let a = &args.args;
    let preset = {
        let v = u(&a[0], vm)?;
        v as i32
    };
    let instr = {
        let v = u(&a[1], vm)?;
        v as i32
    };
    let seed = args
        .kwargs
        .get("seed")
        .map(|o| u(o, vm).map(|v| v as u64))
        .or_else(|| a.get(2).map(|o| u(o, vm).map(|v| v as u64)))
        .transpose()?;
    let play = args
        .kwargs
        .get("play")
        .map(|o| to_bool(o))
        .or_else(|| a.get(3).map(|o| to_bool(o)));
    let result = pyxel::pyxel().gen_bgm(preset, instr, seed, play);
    let list: Vec<PyObjectRef> = result.iter().map(|s| vm.new_pyobj(s.clone())).collect();
    Ok(vm.new_pyobj(list))
}
