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

#[test]
fn collection_sequence_api_script_runs_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-sequences.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(16, 16, headless=True)

base = len(pyxel.colors)
first = pyxel.colors[0]
pyxel.colors[0] = 0x123456
assert pyxel.colors[0] == 0x123456
pyxel.colors.append(0xabcdef)
assert pyxel.colors[-1] == 0xabcdef
pyxel.colors.insert(1, 0x111111)
assert pyxel.colors.pop(1) == 0x111111
pyxel.colors.extend([0x222222, 0x333333])
assert 0x222222 in pyxel.colors
del pyxel.colors[-1]
assert len(pyxel.colors) == base + 2
pyxel.colors[0] = first

image_count = 0
for _ in pyxel.images:
    image_count += 1
assert image_count == pyxel.NUM_IMAGES
assert bool(pyxel.images)
assert pyxel.images[-1].width == pyxel.IMAGE_SIZE

snd = pyxel.Sound()
snd.notes.extend([1, 2])
snd.notes.insert(1, 9)
assert list(snd.notes) == [1, 9, 2]
assert 9 in snd.notes
assert snd.notes == [1, 9, 2]
del snd.notes[1]
assert list(snd.notes) == [1, 2]
assert snd.notes.pop() == 2
assert list(snd.notes) == [1]
snd.notes.clear()
assert not snd.notes

tone = pyxel.Tone()
tone.wavetable.extend([1, 2, 3])
tone.wavetable[1] = 4
assert list(tone.wavetable) == [1, 4, 3]

music = pyxel.Music()
music.set([0], [1])
music.seqs[0] = [2, 3]
assert list(music.seqs[0]) == [2, 3]
music.seqs.insert(1, [4])
assert list(music.seqs[1]) == [4]
assert music.seqs.pop(1)[0] == 4
music.seqs.append([5])
assert music.seqs[-1][0] == 5
del music.seqs[-1]
assert bool(music.seqs)
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
fn integrated_app_script_runs_headless() {
    let dir = std::env::temp_dir();
    let script = dir.join("pyxel-pocket-integrated-app.py");
    let resource = dir.join("pyxel-pocket-integrated-app.pyxres");
    let palette = dir.join("pyxel-pocket-integrated-app.pyxpal");
    let resource_path = resource.to_string_lossy();
    fs::write(
        &script,
        format!(
            "\
import pyxel

RESOURCE = '{}'

pyxel.init(32, 32, headless=True)

pyxel.colors[0] = 0x010203
pyxel.images[0].cls(0)
pyxel.images[0].pset(1, 1, 7)
pyxel.tilemaps[0].refimg = 0
pyxel.tilemaps[0].pset(0, 0, (1, 0))
pyxel.sounds[0].set('c0d0', 'p', '7', 's', 8)
pyxel.musics[0].set([0], [0])
pyxel.save(RESOURCE)
pyxel.save_pal(RESOURCE)

pyxel.colors[0] = 0xffffff
pyxel.images[0].cls(0)
pyxel.tilemaps[0].pset(0, 0, (0, 0))
pyxel.sounds[0].notes.clear()
pyxel.musics[0].seqs.clear()
pyxel.load(RESOURCE)

assert pyxel.colors[0] == 0x010203
assert pyxel.images[0].pget(1, 1) == 7
tile = pyxel.tilemaps[0].pget(0, 0)
assert tile[0] == 1
assert tile[1] == 0
assert pyxel.sounds[0].notes[0] == 0
assert pyxel.sounds[0].notes[1] == 2
assert pyxel.musics[0].seqs[0][0] == 0

state = [False, False]

def update():
    state[0] = True
    if pyxel.frame_count == 0:
        pyxel.play(0, 0)
        pyxel.playm(0)
        pos = pyxel.play_pos(0)
        assert pos == None or pos[0] == 0
        pyxel.stop()
    if pyxel.frame_count >= 2:
        pyxel.quit()

def draw():
    state[1] = True
    pyxel.cls(0)
    pyxel.blt(0, 0, 0, 0, 0, 4, 4)
    assert pyxel.pget(1, 1) == 7
    pyxel.bltm(8, 0, 0, 0, 0, 1, 1)
    pyxel.text(0, 8, 'ok', 7)
    print('integrated-ok')

pyxel.run(update, draw)
",
            resource_path.replace('\\', "\\\\").replace('\'', "\\'")
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    let _ = fs::remove_file(script);
    let _ = fs::remove_file(resource);
    let _ = fs::remove_file(palette);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("integrated-ok"),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn example_style_class_app_script_runs_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-class-app.py");
    fs::write(
        &script,
        "\
import pyxel

class App:
    def __init__(self):
        pyxel.init(48, 32, title='Pocket Class App', headless=True)
        pyxel.mouse(True)
        pyxel.images[0].set(0, 0, [
            '0770',
            '7667',
            '7667',
            '0770',
        ])
        pyxel.tilemaps[0].set(0, 0, [
            '0000 0100',
            '0100 0000',
        ])
        pyxel.tilemaps[0].imgsrc = 0
        pyxel.sounds[0].set(
            notes='c0d0',
            tones='p',
            volumes='7',
            effects='s',
            speed=8,
        )
        self.did_update = False
        self.did_draw = False
        pyxel.run(self.update, self.draw)

    def update(self):
        self.did_update = True
        if pyxel.frame_count == 0:
            pyxel.play(0, 0)
        if pyxel.frame_count >= 2:
            pyxel.stop()
            pyxel.quit()

    def draw(self):
        self.did_draw = True
        pyxel.cls(0)
        pyxel.clip()
        pyxel.camera()
        pyxel.pal(7, 10)
        pyxel.dither(1.0)
        pyxel.rect(1, 1, 6, 4, 1)
        pyxel.rectb(0, 0, 10, 8, 7)
        pyxel.circ(18, 4, 3, 8)
        pyxel.circb(28, 4, 3, 9)
        pyxel.line(0, 12, 12, 20, 11)
        pyxel.tri(16, 16, 20, 10, 24, 16, 12)
        pyxel.trib(28, 16, 32, 10, 36, 16, 13)
        pyxel.blt(2, 22, 0, 0, 0, 4, 4, 0)
        pyxel.bltm(12, 22, 0, 0, 0, 2, 2)
        pyxel.text(22, 22, 'ok', 7)
        assert pyxel.sin(90) == 1
        assert pyxel.cos(0) == 1
        if self.did_update and self.did_draw:
            print('class-app-ok')

App()
",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    let _ = fs::remove_file(script);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("class-app-ok"),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn real_asset_loading_script_runs_headless() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let assets = root.join("examples/assets");
    let cat = assets.join("cat_16x16.png").to_string_lossy().into_owned();
    let audio_image = assets.join("audio_bgm1.png").to_string_lossy().into_owned();
    let audio_pcm = assets.join("audio_bgm1.ogg").to_string_lossy().into_owned();
    let palette = assets
        .join("audio_bgm.pyxpal")
        .to_string_lossy()
        .into_owned();
    let tmx = assets.join("urban_rpg.tmx").to_string_lossy().into_owned();
    let sample = assets.join("sample.pyxres").to_string_lossy().into_owned();
    let script = std::env::temp_dir().join("pyxel-pocket-real-assets.py");
    fs::write(
        &script,
        format!(
            "\
import pyxel

CAT = '{}'
AUDIO_IMAGE = '{}'
AUDIO_PCM = '{}'
PALETTE = '{}'
TMX = '{}'
SAMPLE = '{}'

pyxel.init(64, 64, headless=True)
pyxel.load_pal(PALETTE)

pyxel.images[0].load(0, 0, CAT, include_colors=True)
seen_pixel = False
for y in range(16):
    for x in range(16):
        if pyxel.images[0].pget(x, y) != 0:
            seen_pixel = True
assert seen_pixel

image = pyxel.Image.from_image(AUDIO_IMAGE)
assert image.width > 0
assert image.height > 0
pyxel.images[1] = image

tilemap = pyxel.Tilemap.from_tmx(TMX, 0)
assert tilemap.width > 0
assert tilemap.height > 0
pyxel.tilemaps[0] = tilemap
tile = pyxel.tilemaps[0].pget(0, 0)
assert len(tile) == 2

pyxel.sounds[0].pcm(AUDIO_PCM)
assert pyxel.sounds[0].total_sec() > 0
pyxel.channels[0].gain = 0.5
assert pyxel.channels[0].gain == 0.5
pyxel.play(0, 0)
pos = pyxel.play_pos(0)
assert pos == None or pos[0] == 0
pyxel.stop()

pyxel.load(SAMPLE)
pyxel.blt(0, 0, 0, 0, 0, 8, 8)
print('real-assets-ok')
",
            escape_python_path(&cat),
            escape_python_path(&audio_image),
            escape_python_path(&audio_pcm),
            escape_python_path(&palette),
            escape_python_path(&tmx),
            escape_python_path(&sample),
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    let _ = fs::remove_file(script);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("real-assets-ok"),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn keyword_init_script_runs_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-keywords.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(width=16, height=16, title='Keyword Init', headless=True)
assert pyxel.width == 16
assert pyxel.height == 16
print('keyword-init-ok')
",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    let _ = fs::remove_file(script);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("keyword-init-ok"),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn escape_python_path(path: &str) -> String {
    path.replace('\\', "\\\\").replace('\'', "\\'")
}
