use std::fs;
use std::process::Command;

#[test]
fn exec_source_accepts_simple_python() {
    pyxel_pocket::Runtime::new()
        .exec_source("x = 1 + 2", "<test>")
        .unwrap();
}

#[test]
fn pyxel_module_exposes_constants_without_initializing_core() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import pyxel
assert pyxel.KEY_Q >= 0
assert pyxel.COLOR_WHITE >= 0
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn binary_runs_script_file() {
    let script = std::env::temp_dir().join("pyxel-pocket-smoke.py");
    fs::write(&script, "import pyxel\npyxel.init(8, 8, headless=True)\n").unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .status()
        .unwrap();

    assert!(status.success());
    let _ = fs::remove_file(script);
}

#[test]
fn mvp_api_script_runs_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-mvp.py");
    fs::write(
        &script,
        "\
import pyxel
pyxel.init(16, 16, headless=True)
pyxel.cls(pyxel.COLOR_BLACK)
pyxel.pset(1, 2, pyxel.COLOR_WHITE)
pyxel.line(0, 0, 3, 3, pyxel.COLOR_RED)
pyxel.rect(1, 1, 4, 4, pyxel.COLOR_WHITE)
pyxel.rectb(0, 0, 8, 8, pyxel.COLOR_RED)
pyxel.text(0, 0, 'ok', pyxel.COLOR_WHITE)
assert pyxel.width == 16
assert pyxel.height == 16
assert pyxel.btn(pyxel.KEY_Q) == False
assert pyxel.btnp(pyxel.KEY_Q) == False
assert pyxel.btnr(pyxel.KEY_Q) == False
",
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .status()
        .unwrap();

    assert!(status.success());
    let _ = fs::remove_file(script);
}

#[test]
fn run_executes_update_and_draw_callbacks() {
    let script = std::env::temp_dir().join("pyxel-pocket-run.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(16, 16, headless=True)

def update():
    if pyxel.frame_count >= 2:
        pyxel.quit()

def draw():
    pyxel.cls(pyxel.COLOR_BLACK)
    pyxel.text(0, 0, 'ok', pyxel.COLOR_WHITE)
    print('draw-called')

pyxel.run(update, draw)
",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("draw-called"));
    let _ = fs::remove_file(script);
}

#[test]
fn class_and_collection_api_script_runs_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-objects.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(16, 16, headless=True)

img = pyxel.Image(8, 8)
img.cls(pyxel.COLOR_BLACK)
img.pset(1, 2, pyxel.COLOR_WHITE)
assert img.pget(1, 2) == pyxel.COLOR_WHITE
src = pyxel.Image(8, 8)
src.pset(0, 0, pyxel.COLOR_RED)
img.blt(3, 3, src, 0, 0, 1, 1)
assert img.pget(3, 3) == pyxel.COLOR_RED

pyxel.images[0].pset(0, 0, pyxel.COLOR_RED)
assert pyxel.images[0].pget(0, 0) == pyxel.COLOR_RED

tm = pyxel.Tilemap(8, 8, 0)
tm.pset(1, 1, (2, 3))
tile = tm.pget(1, 1)
assert tile[0] == 2
assert tile[1] == 3
tm.refimg = 0
assert tm.refimg == 0

snd = pyxel.Sound()
snd.set_notes('c0d0')
assert len(snd.notes) == 2
snd.notes.append(7)
assert snd.notes[2] == 7
pyxel.sounds[0] = snd
assert pyxel.sounds[0].notes[2] == 7

msc = pyxel.Music()
msc.set([0, 1], [], [2])
assert len(msc.seqs) >= 3
assert len(msc.seqs[0]) == 2
seq_count = len(msc.seqs)
msc.seqs.append([3])
assert msc.seqs[seq_count][0] == 3

ch = pyxel.Channel()
assert ch.play_pos() == None

tone = pyxel.Tone()
tone.wavetable.append(1)
assert len(tone.wavetable) == 1
",
    )
    .unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .status()
        .unwrap();

    assert!(status.success());
    let _ = fs::remove_file(script);
}
