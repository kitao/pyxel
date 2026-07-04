use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::{exec_source_in_current_runtime, normalize_source, Runtime};

static NEXT_EXTRACT_ID: AtomicU64 = AtomicU64::new(0);
static RESTART_COMMAND: Mutex<Option<RestartCommand>> = Mutex::new(None);
static LIVE_EXTRACTED_APPS: Mutex<Vec<ExtractedApp>> = Mutex::new(Vec::new());

#[derive(Clone)]
struct RestartCommand {
    program: PathBuf,
    args: Vec<OsString>,
    cwd: PathBuf,
}

impl RestartCommand {
    fn spawn_and_exit(&self) {
        if let Err(err) = Command::new(&self.program)
            .args(&self.args)
            .current_dir(&self.cwd)
            .spawn()
        {
            eprintln!("failed to restart Pyxel app: {err}");
            process::exit(1);
        }
        process::exit(0);
    }
}

struct ExtractedApp {
    dir: PathBuf,
    startup_script: PathBuf,
}

impl Drop for ExtractedApp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

pub fn run_path(path: &Path) -> Result<(), String> {
    capture_restart_command()?;
    let _runtime = Runtime::new();
    if is_pyxapp_path(path) {
        play_pyxapp_in_current_runtime(path)
    } else {
        run_script_in_current_runtime(path)
    }
}

pub(crate) fn play_pyxapp_in_current_runtime(path: &Path) -> Result<(), String> {
    let app = extract_pyxapp(path)?;
    let result = run_script_in_current_runtime(&app.startup_script);
    if result.is_ok() {
        LIVE_EXTRACTED_APPS
            .lock()
            .expect("PocketPy extracted app lock poisoned")
            .push(app);
    }
    result
}

pub(crate) fn install_reset_callback() {
    let restart_command = RESTART_COMMAND
        .lock()
        .expect("PocketPy restart command lock poisoned")
        .clone();
    if let Some(restart_command) = restart_command {
        *pyxel::reset_callback() = Some(Box::new(move || restart_command.spawn_and_exit()));
    }
}

pub(crate) fn clear_extracted_apps() {
    LIVE_EXTRACTED_APPS
        .lock()
        .expect("PocketPy extracted app lock poisoned")
        .clear();
}

fn capture_restart_command() -> Result<(), String> {
    let mut raw_args = std::env::args_os();
    let Some(raw_program) = raw_args.next() else {
        return Ok(());
    };
    let program = std::env::current_exe().unwrap_or_else(|_| PathBuf::from(raw_program));
    let command = RestartCommand {
        program,
        args: raw_args.collect(),
        cwd: std::env::current_dir().map_err(|err| format!("cannot read current dir: {err}"))?,
    };
    *RESTART_COMMAND
        .lock()
        .expect("PocketPy restart command lock poisoned") = Some(command);
    Ok(())
}

fn run_script_in_current_runtime(path: &Path) -> Result<(), String> {
    let path = fs::canonicalize(path)
        .map_err(|err| format!("cannot resolve '{}': {err}", path.display()))?;
    let source = fs::read_to_string(&path)
        .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;

    if let Some(parent) = path.parent() {
        std::env::set_current_dir(parent)
            .map_err(|err| format!("cannot enter '{}': {err}", parent.display()))?;
    }

    let file_path = python_string_literal(&path.to_string_lossy());
    exec_source_in_current_runtime(&format!("__file__ = '{file_path}'"), "<setup>")?;
    exec_source_in_current_runtime(
        &source,
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<script>"),
    )
}

fn extract_pyxapp(path: &Path) -> Result<ExtractedApp, String> {
    let path = complete_pyxapp_path(path)?;
    let file = File::open(&path).map_err(|_| format!("no such file: '{}'", path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|_| format!("failed to parse file: '{}'", path.display()))?;

    let dir = create_extract_dir()?;
    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|err| format!("failed to read Pyxel app entry: {err}"))?;
        let name = file.name().to_owned();
        let Some(enclosed_name) = file.enclosed_name() else {
            return Err(format!("unsafe path in Pyxel app: '{name}'"));
        };
        let target = dir.join(enclosed_name);
        if file.is_dir() {
            fs::create_dir_all(&target)
                .map_err(|err| format!("cannot create '{}': {err}", target.display()))?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("cannot create '{}': {err}", parent.display()))?;
        }
        if is_python_path(&target) {
            let mut source = String::new();
            file.read_to_string(&mut source)
                .map_err(|err| format!("cannot read '{}': {err}", target.display()))?;
            fs::write(&target, normalize_source(&source))
                .map_err(|err| format!("cannot write '{}': {err}", target.display()))?;
        } else {
            let mut output = File::create(&target)
                .map_err(|err| format!("cannot create '{}': {err}", target.display()))?;
            io::copy(&mut file, &mut output)
                .map_err(|err| format!("cannot extract '{}': {err}", target.display()))?;
        }
    }

    let Some(startup_script) = find_startup_script(&dir)? else {
        return Err(format!(
            "file not found: '{}'",
            pyxel::APP_STARTUP_SCRIPT_FILE
        ));
    };
    Ok(ExtractedApp {
        dir,
        startup_script,
    })
}

fn complete_pyxapp_path(path: &Path) -> Result<PathBuf, String> {
    let path = if path.extension().is_none() {
        path.with_extension(pyxel::APP_FILE_EXTENSION.trim_start_matches('.'))
    } else {
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if extension != "zip" && extension != pyxel::APP_FILE_EXTENSION.trim_start_matches('.') {
            return Err(format!(
                "'play' command only accepts {} files",
                pyxel::APP_FILE_EXTENSION
            ));
        }
        path.to_path_buf()
    };
    fs::canonicalize(&path).map_err(|err| format!("cannot resolve '{}': {err}", path.display()))
}

fn find_startup_script(dir: &Path) -> Result<Option<PathBuf>, String> {
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        for entry in fs::read_dir(&current_dir)
            .map_err(|err| format!("cannot read '{}': {err}", current_dir.display()))?
        {
            let entry = entry.map_err(|err| format!("cannot read directory entry: {err}"))?;
            let path = entry.path();
            if entry
                .file_type()
                .map_err(|err| format!("cannot stat '{}': {err}", path.display()))?
                .is_dir()
            {
                dirs.push(path);
                continue;
            }
            if path.file_name().and_then(|name| name.to_str())
                == Some(pyxel::APP_STARTUP_SCRIPT_FILE)
            {
                let startup = fs::read_to_string(&path)
                    .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;
                let startup = path.parent().unwrap_or(dir).join(startup.trim());
                return Ok(Some(startup));
            }
        }
    }
    Ok(None)
}

fn create_extract_dir() -> Result<PathBuf, String> {
    let count = NEXT_EXTRACT_ID.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| format!("system clock error: {err}"))?
        .as_nanos();
    let dir = std::env::temp_dir()
        .join(pyxel::BASE_DIR)
        .join("pocket-play")
        .join(format!("{}_{}_{}", std::process::id(), nanos, count));
    fs::create_dir_all(&dir).map_err(|err| format!("cannot create '{}': {err}", dir.display()))?;
    Ok(dir)
}

fn is_pyxapp_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("pyxapp"))
}

fn is_python_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("py"))
}

fn python_string_literal(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "\\'")
}
