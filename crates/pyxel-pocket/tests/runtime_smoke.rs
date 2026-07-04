use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};
use std::{fs, thread};

use zip::write::SimpleFileOptions;

#[test]
fn exec_source_accepts_simple_python() {
    pyxel_pocket::Runtime::new()
        .exec_source("x = 1 + 2", "<test>")
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_boolean_continuation() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
ok = (
    True
    and True
    and not False
)
assert ok
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_adjacent_strings() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
value = 2
text = (
    f'value={value}'
    ' ok'
)
assert text == 'value=2 ok'
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_upper_hex_format_specifier() {
    pyxel_pocket::Runtime::new()
        .exec_source("assert f'{255:06X}' == '0000FF'", "<test>")
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_leading_plus_continuation() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
text = (
    'hello'
    + ' pocket'
)
assert text == 'hello pocket'
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_leading_comparison_continuation() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
ok = (
    1
    == 1
)
assert ok
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_trailing_boolean_continuation() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
ok = (
    True and
    True and
    not False
)
assert ok
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_trailing_boolean_continuation_with_space() {
    pyxel_pocket::Runtime::new()
        .exec_source("ok = (\n    True and \n    True\n)\nassert ok\n", "<test>")
        .unwrap();
}

#[test]
fn exec_source_accepts_dict_key_value_continuation() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
data = {
    'notes':
        'ab'
        'cd'
        ,
}
assert data['notes'] == 'abcd'
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_for_loop_unpacking() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
total = 0
for i, (x, y) in enumerate([(2, 3)]):
    total += i + x + y
assert total == 5
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_deque_indexing() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from collections import deque

d = deque([(1, 2), (3, 4)])
assert d[0] == (1, 2)
assert d[-1] == (3, 4)
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_nested_list_comprehension_assignment() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
pairs = [(x, y) for x in range(2) for y in range(2)]
assert len(pairs) == 4
assert pairs[0] == (0, 0)
assert pairs[3] == (1, 1)
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_whole_list_slice_assignment() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
items = [1, 2, 3]
items[:] = [item for item in items if item > 1]
assert items == [2, 3]
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_float_floor_division() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
assert 5.5 // 2 == 2.0
assert 5 // 2.0 == 2.0
assert -1.0 // 2 == -1.0
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_multiline_any_generator_expression() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
values = [1, 2, 3]
if any(
    value == 2
    for value in values
):
    matched = True
else:
    matched = False
assert matched
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_multiline_enumerate_generator_expression() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
metadata = {'name': 'skip', 'title': 'Hello', 'author': 'Pyxel'}
values = []
for i, key in enumerate(
    key for key in metadata if key != 'name'
):
    values.append((i, key))
assert values == [(0, 'title'), (1, 'author')]
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_sum_generator_expression() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
values = [1, 2, 3]
total = sum(value for value in values)
assert total == 6
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_parenthesized_from_import() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from math import (
    sin,
    cos,
)
assert sin(0) == 0
assert cos(0) == 1
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_unary_plus_identifier() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
value = 3
assert +value == 3
pair = (-value, +value)
assert pair == (-3, 3)
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_named_default_arguments() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
DEFAULT_COLOR = 7

def color(value=DEFAULT_COLOR):
    return value

assert color() == 7
assert color(3) == 3
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn module_blit_functions_accept_resource_instances() {
    let script = std::env::temp_dir().join("pyxel-pocket-module-blit-resources.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(8, 8, headless=True)
image = pyxel.Image(8, 8)
image.pset(0, 0, 7)
pyxel.blt(0, 0, image, 0, 0, 1, 1)
assert pyxel.pget(0, 0) == 7

tilemap = pyxel.Tilemap(8, 8, image)
tilemap.pset(0, 0, (0, 0))
pyxel.bltm(1, 0, tilemap, 0, 0, 1, 1)
assert pyxel.pget(1, 0) == 7

pyxel.blt3d(0, 1, 4, 4, image, (0, 0, 1), (0, 0, 0))
pyxel.bltm3d(4, 1, 4, 4, tilemap, (0, 0, 1), (0, 0, 0))
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
}

#[test]
fn image_set_accepts_tuple_of_strings() {
    let script = unique_temp_path("pyxel-pocket-image-set-tuple", "py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(8, 8, headless=True)
image = pyxel.Image(8, 8)
image.set(0, 0, (
    '10000000',
    '01000000',
    '00100000',
    '00010000',
    '00001000',
    '00000100',
    '00000010',
    '00000001',
))
assert image.pget(0, 0) == 1
assert image.pget(7, 7) == 1
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
}

#[test]
fn audio_play_accepts_deprecated_tick_keyword() {
    let script = unique_temp_path("pyxel-pocket-play-tick", "py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(8, 8, headless=True)
pyxel.play(0, 0, tick=0, loop=True)
pyxel.playm(0, tick=0, loop=True)
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
}

#[test]
fn pyxel_cli_metadata_reads_pyxapp_comments() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let app = root
        .join("python/pyxel/examples/apps")
        .join("megaball.pyxapp")
        .to_string_lossy()
        .into_owned();
    let script = std::env::temp_dir().join("pyxel-pocket-cli-metadata.py");
    fs::write(
        &script,
        format!(
            "\
import pyxel
import pyxel.cli

metadata = pyxel.cli.get_pyxel_app_metadata('{}')
assert metadata['title'] == 'Megaball'
assert metadata['author'] == 'Adam'
assert metadata['license'] == 'MIT'
",
            escape_python_path(&app),
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
}

#[test]
fn pyxel_cli_play_pyxapp_runs_startup_script() {
    let pyxapp = unique_temp_path("pyxel-pocket-play", "pyxapp");
    write_test_pyxapp(
        &pyxapp,
        &[
            ("sample/.pyxapp_startup_script", "sample/main.py"),
            ("sample/sample/helper.py", "VALUE = 'from-helper'\n"),
            (
                "sample/sample/main.py",
                "\
import helper
import pyxel

pyxel.init(8, 8, headless=True)
print('pyxapp-started:' + helper.VALUE)
",
            ),
        ],
    );

    let script = unique_temp_path("pyxel-pocket-play-script", "py");
    fs::write(
        &script,
        format!(
            "\
import pyxel
import pyxel.cli

pyxel.cli.play_pyxel_app('{}')
",
            escape_python_path(&pyxapp.to_string_lossy()),
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .env("PYXEL_POCKET_HEADLESS", "1")
        .output()
        .unwrap();

    let _ = fs::remove_file(script);
    let _ = fs::remove_file(pyxapp);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("pyxapp-started:from-helper"),
        "stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn binary_runs_pyxapp_file() {
    let pyxapp = unique_temp_path("pyxel-pocket-binary-play", "pyxapp");
    write_test_pyxapp(
        &pyxapp,
        &[
            ("direct/.pyxapp_startup_script", "direct/main.py"),
            (
                "direct/direct/main.py",
                "\
import pyxel

pyxel.init(8, 8, headless=True)
print('direct-pyxapp-started')
",
            ),
        ],
    );

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&pyxapp)
        .env("PYXEL_POCKET_HEADLESS", "1")
        .output()
        .unwrap();

    let _ = fs::remove_file(pyxapp);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("direct-pyxapp-started"),
        "stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn pyxapp_imported_modules_are_normalized_before_execution() {
    let pyxapp = unique_temp_path("pyxel-pocket-normalized-import", "pyxapp");
    write_test_pyxapp(
        &pyxapp,
        &[
            ("normalized/.pyxapp_startup_script", "normalized/main.py"),
            (
                "normalized/normalized/helper.py",
                "\
OK = (
    False
    or True
)
",
            ),
            (
                "normalized/normalized/main.py",
                "\
import helper
import pyxel

pyxel.init(8, 8, headless=True)
assert helper.OK
print('normalized-import-started')
",
            ),
        ],
    );

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&pyxapp)
        .env("PYXEL_POCKET_HEADLESS", "1")
        .output()
        .unwrap();

    let _ = fs::remove_file(pyxapp);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("normalized-import-started"),
        "stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn exec_source_accepts_minimal_pathlib_path() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from pathlib import Path

base = Path('/tmp/app/main.py').parent / 'assets'
assert str(base) == '/tmp/app/assets'
assert str(Path('relative').resolve()).endswith('/relative')
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_os_environ_mapping() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import os

assert os.environ.pop('PYXEL_POCKET_MISSING', None) == None
os.environ['PYXEL_POCKET_VALUE'] = 'ok'
assert os.environ.pop('PYXEL_POCKET_VALUE', None) == 'ok'
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_str_capitalize() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
assert 'title'.capitalize() == 'Title'
assert 'tITLE'.capitalize() == 'Title'
assert ''.capitalize() == ''
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_random_sample() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from random import sample

values = sample([1, 2, 3], 2)
assert len(values) == 2
assert values[0] in [1, 2, 3]
assert values[1] in [1, 2, 3]
assert values[0] != values[1]
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_enum_auto() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from enum import IntEnum, auto

class Kind(IntEnum):
    A = auto()
    B = auto()

assert Kind.A != Kind.B
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_accepts_itertools_filterfalse() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from itertools import filterfalse

values = [1, 2, 3]
values[:] = filterfalse(lambda value: value == 2, values)
assert values == [1, 3]
",
            "<test>",
        )
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
fn object_collection_sequence_methods_run_headless() {
    let script = std::env::temp_dir().join("pyxel-pocket-object-collections.py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(16, 16, headless=True)

ch0 = pyxel.Channel()
ch0.gain = 0.25
ch1 = pyxel.Channel()
ch1.gain = 0.75
pyxel.channels[:] = [ch0, ch1]
assert len(pyxel.channels) == 2
assert pyxel.channels[0].gain == 0.25
assert pyxel.channels[1].gain == 0.75

ch2 = pyxel.Channel()
ch2.gain = 0.5
pyxel.channels.insert(1, ch2)
assert pyxel.channels[1].gain == 0.5
assert pyxel.channels.pop(1).gain == 0.5

tone = pyxel.Tone()
tone.wavetable[:] = [1, 2, 3]
pyxel.tones[:] = [tone]
assert len(pyxel.tones) == 1
assert list(pyxel.tones[0].wavetable) == [1, 2, 3]
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

#[test]
fn shipped_examples_run_headless_with_runner_limits() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    for name in [
        "01_hello_pyxel.py",
        "02_jump_game.py",
        "03_draw_api.py",
        "04_sound_api.py",
        "05_color_palette.py",
        "06_click_game.py",
        "07_snake.py",
        "08_triangle_api.py",
        "09_shooter.py",
        "10_platformer.py",
        "11_offscreen.py",
        "12_perlin_noise.py",
        "13_custom_font.py",
        "14_synthesizer.py",
        "15_tiled_map_file.py",
        "16_transform.py",
        "17_app_launcher.py",
        "18_audio_playback.py",
        "19_perspective.py",
        "99_flip_animation.py",
    ] {
        let example = root.join("python/pyxel/examples").join(name);
        let mut command = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"));
        command
            .arg(&example)
            .env("PYXEL_POCKET_HEADLESS", "1")
            .env("PYXEL_POCKET_MAX_FRAMES", "3");

        let output = run_with_timeout(&mut command, Duration::from_secs(4))
            .unwrap_or_else(|err| panic!("{name}: {err}"));

        assert!(
            output.status.success(),
            "{name}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn app_launcher_example_runs_headless_with_runner_limits() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let example = root.join("python/pyxel/examples/17_app_launcher.py");
    let mut command = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"));
    command
        .arg(&example)
        .env("PYXEL_POCKET_HEADLESS", "1")
        .env("PYXEL_POCKET_MAX_FRAMES", "3");

    let output = run_with_timeout(&mut command, Duration::from_secs(4)).unwrap();

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn shipped_pyxapps_run_headless_with_runner_limits() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    for name in [
        "30sec_of_daylight.pyxapp",
        "cursed_caverns.pyxapp",
        "laser-jetman.pyxapp",
        "mega_wing.pyxapp",
        "megaball.pyxapp",
        "space_rescue.pyxapp",
        "vortexion.pyxapp",
    ] {
        let app = root.join("python/pyxel/examples/apps").join(name);
        let mut command = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"));
        command
            .arg(&app)
            .env("PYXEL_POCKET_HEADLESS", "1")
            .env("PYXEL_POCKET_MAX_FRAMES", "3");

        let output = run_with_timeout(&mut command, Duration::from_secs(5))
            .unwrap_or_else(|err| panic!("{name}: {err}"));

        assert!(
            output.status.success(),
            "{name}\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn run_with_timeout(command: &mut Command, timeout: Duration) -> Result<Output, String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to spawn command: {err}"))?;
    let start = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(_)) => return child.wait_with_output().map_err(|err| err.to_string()),
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let output = child
                    .wait_with_output()
                    .map_err(|err| format!("failed to collect timed-out command: {err}"))?;
                return Err(format!(
                    "command timed out after {:?}\nstdout:\n{}\nstderr:\n{}",
                    timeout,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(err) => return Err(format!("failed to wait for command: {err}")),
        }
    }
}

fn escape_python_path(path: &str) -> String {
    path.replace('\\', "\\\\").replace('\'', "\\'")
}

fn unique_temp_path(prefix: &str, extension: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "{prefix}-{}-{nanos}.{extension}",
        std::process::id()
    ))
}

fn write_test_pyxapp(path: &std::path::Path, files: &[(&str, &str)]) {
    let file = fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    for (name, content) in files {
        zip.start_file(*name, SimpleFileOptions::default()).unwrap();
        std::io::Write::write_all(&mut zip, content.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}
