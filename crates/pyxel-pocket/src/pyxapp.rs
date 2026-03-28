use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const STARTUP_SCRIPT_MARKER: &str = ".pyxapp_startup_script";

/// Extract a .pyxapp (zip) archive to a temp directory and return the
/// path to the startup script.
pub fn extract_and_find_startup(pyxapp_path: &Path) -> Result<(PathBuf, PathBuf), String> {
    let file = fs::File::open(pyxapp_path)
        .map_err(|e| format!("Cannot open '{}': {e}", pyxapp_path.display()))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Invalid zip archive '{}': {e}", pyxapp_path.display()))?;

    let temp_dir = std::env::temp_dir().join("pyxel_pocket_play");
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir).map_err(|e| format!("Cannot create temp dir: {e}"))?;

    // Extract all files
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| format!("Zip error: {e}"))?;
        let entry_path = temp_dir.join(entry.name());

        if entry.is_dir() {
            fs::create_dir_all(&entry_path)
                .map_err(|e| format!("Cannot create dir '{}': {e}", entry_path.display()))?;
        } else {
            if let Some(parent) = entry_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Cannot create dir '{}': {e}", parent.display()))?;
            }
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| format!("Read error: {e}"))?;
            fs::write(&entry_path, &buf)
                .map_err(|e| format!("Write error '{}': {e}", entry_path.display()))?;
        }
    }

    // Find .pyxapp_startup_script marker
    find_startup_script(&temp_dir)
        .map(|script_path| (temp_dir, script_path))
        .ok_or_else(|| format!("'{}' not found in archive", STARTUP_SCRIPT_MARKER))
}

fn find_startup_script(base_dir: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(base_dir).ok()?;
    for entry in entries.flatten() {
        let marker = entry.path().join(STARTUP_SCRIPT_MARKER);
        if marker.is_file() {
            let relative_script = fs::read_to_string(&marker).ok()?.trim().to_string();
            let script_path = entry.path().join(&relative_script);
            if script_path.is_file() {
                return Some(script_path);
            }
        }
    }
    None
}

/// Clean up the temporary extraction directory.
pub fn cleanup() {
    let temp_dir = std::env::temp_dir().join("pyxel_pocket_play");
    let _ = fs::remove_dir_all(temp_dir);
}
