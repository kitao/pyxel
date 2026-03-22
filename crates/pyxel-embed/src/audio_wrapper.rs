use rustpython_vm::builtins::PyList;
use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

use crate::helpers::*;
use crate::sound_wrapper::PySound;

pub fn play(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let ch = u(&a[0], vm)?;
    let snd_obj = &a[1];
    let sec = kw_f32(&args, "sec", vm)?.or_else(|| args.args.get(2).and_then(|o| f(o, vm).ok()));
    let loop_ = args
        .kwargs
        .get("loop")
        .map(to_bool)
        .or_else(|| ob(a, 3, vm))
        .unwrap_or(false);
    let resume = args
        .kwargs
        .get("resume")
        .map(to_bool)
        .or_else(|| ob(a, 4, vm))
        .unwrap_or(false);

    // snd: int, list of int, Sound, list of Sound, or str (MML)
    if let Ok(snd) = u(snd_obj, vm) {
        pyxel::pyxel().play_sound(ch, snd, sec, loop_, resume);
    } else if let Some(sound) = snd_obj.payload::<PySound>() {
        unsafe { &mut *pyxel::channels()[ch as usize] }.play_sound(sound.inner, sec, loop_, resume);
    } else if let Some(list) = snd_obj.payload::<PyList>() {
        let items = list.borrow_vec();
        // Try as list of Sound objects first, then list of int
        if items
            .first()
            .is_some_and(|o| o.payload::<PySound>().is_some())
        {
            let sounds: Vec<*mut pyxel::Sound> = items
                .iter()
                .map(|item| {
                    item.payload::<PySound>()
                        .map(|s| s.inner)
                        .ok_or_else(|| vm.new_type_error("expected Sound in list".into()))
                })
                .collect::<PyResult<_>>()?;
            unsafe { &mut *pyxel::channels()[ch as usize] }.play(sounds, sec, loop_, resume);
        } else {
            let snd_list: Vec<u32> = items
                .iter()
                .map(|item| u(item, vm))
                .collect::<PyResult<_>>()?;
            pyxel::pyxel().play(ch, &snd_list, sec, loop_, resume);
        }
    } else if let Some(code) = s(snd_obj) {
        pyxel::pyxel()
            .play_mml(ch, code, sec, loop_, resume)
            .map_err(|e| vm.new_value_error(e))?;
    } else {
        return Err(vm.new_type_error("expected int, list, Sound, or str for snd".into()));
    }
    Ok(())
}

pub fn playm(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let msc = u(&a[0], vm)?;
    let sec = kw_f32(&args, "sec", vm)?.or_else(|| {
        a.get(1)
            .and_then(|o| if vm.is_none(o) { None } else { f(o, vm).ok() })
    });
    let loop_ = args
        .kwargs
        .get("loop")
        .map(to_bool)
        .or_else(|| ob(a, 2, vm))
        .unwrap_or(false);
    pyxel::pyxel().play_music(msc, sec, loop_);
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
        .map(to_bool)
        .or_else(|| a.get(3).map(to_bool));
    let result = pyxel::pyxel().gen_bgm(preset, instr, seed, play);
    let list: Vec<PyObjectRef> = result.iter().map(|s| vm.new_pyobj(s.clone())).collect();
    Ok(vm.new_pyobj(list))
}
