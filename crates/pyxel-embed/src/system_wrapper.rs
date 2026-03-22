use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::OnceLock;

use rustpython_vm::function::FuncArgs;
use rustpython_vm::{PyObjectRef, PyResult, VirtualMachine};

use crate::helpers::*;

// Process-level original cwd, captured once before any chdir
static ORIGINAL_CWD: OnceLock<PathBuf> = OnceLock::new();

pub fn set_original_cwd() {
    ORIGINAL_CWD.get_or_init(|| std::env::current_dir().unwrap_or_default());
}

// Change to the directory of the calling script (like pyxel-binding's
// `os.chdir(os.path.dirname(inspect.stack()[1].filename) or ".")`)
fn chdir_to_script_dir(vm: &VirtualMachine) {
    // Walk up the call stack to find __file__ in globals
    if let Some(frame) = vm.current_frame() {
        if let Ok(file_obj) = frame.globals.get_item("__file__", vm) {
            if let Some(path_str) = s(&file_obj) {
                if let Some(dir) = Path::new(path_str).parent() {
                    if !dir.as_os_str().is_empty() {
                        let _ = std::env::set_current_dir(dir);
                    }
                }
            }
        }
    }
}

fn register_reset_callback() {
    let exe = std::env::current_exe().unwrap_or_default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let orig_cwd = ORIGINAL_CWD
        .get()
        .cloned()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    *pyxel::reset_callback() = Some(Box::new(move || {
        // Support launcher watch mode
        if std::env::var("PYXEL_WATCH_STATE_FILE").is_ok() {
            exit(0x52);
        }

        // macOS: suppress stderr noise from child process
        #[cfg(target_os = "macos")]
        {
            use std::os::unix::io::AsRawFd;
            if let Ok(devnull) = std::fs::File::open("/dev/null") {
                unsafe {
                    libc::dup2(devnull.as_raw_fd(), 2);
                }
            }
        }

        // Collect env vars, excluding WINDOW_STATE_ENV if it was removed from
        // Python's os.environ (pyxel-core re-sets it via set_var every frame,
        // so we must check whether the Python side intended to remove it).
        // The convention is: if the launched app should NOT inherit the window
        // state, WINDOW_STATE_ENV is absent from Python's os.environ.  Since
        // we can't access the Python VM here, we remove it unconditionally —
        // the same approach as pyxel-binding where os.environ.copy() is used
        // after the pop.
        let env: Vec<(String, String)> = std::env::vars()
            .filter(|(k, _)| k != pyxel::WINDOW_STATE_ENV)
            .collect();

        let _ = Command::new(&exe)
            .args(&args)
            .current_dir(&orig_cwd)
            .env_clear()
            .envs(env)
            .spawn();
        exit(0);
    }));
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

    // Change to script directory so relative resource paths work
    chdir_to_script_dir(vm);

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
            if let Err(exc) = self.update.call((), self.vm) {
                self.vm.print_exception(exc);
                exit(1);
            }
        }

        fn draw(&mut self, _pyxel: &mut pyxel::Pyxel) {
            if let Err(exc) = self.draw.call((), self.vm) {
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

pub fn title(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let title = s(&args.args[0]).ok_or_else(|| vm.new_type_error("expected str".into()))?;
    pyxel::pyxel().set_title(title);
    Ok(())
}

pub fn icon(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    let a = &args.args;
    let data = crate::image_wrapper::extract_str_vec(&a[0], vm)?;
    let scale = u(&a[1], vm)?;
    let colkey = oc(a, 2, vm)?;
    let data_refs: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
    pyxel::pyxel().set_icon(&data_refs, scale, colkey);
    Ok(())
}

pub fn perf_monitor(args: FuncArgs, _vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().set_perf_monitor(to_bool(&args.args[0]));
    Ok(())
}

pub fn integer_scale(args: FuncArgs, _vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().set_integer_scale(to_bool(&args.args[0]));
    Ok(())
}

pub fn screen_mode(args: FuncArgs, vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().set_screen_mode(u(&args.args[0], vm)?);
    Ok(())
}

pub fn fullscreen(args: FuncArgs, _vm: &VirtualMachine) -> PyResult<()> {
    pyxel::pyxel().set_fullscreen(to_bool(&args.args[0]));
    Ok(())
}
