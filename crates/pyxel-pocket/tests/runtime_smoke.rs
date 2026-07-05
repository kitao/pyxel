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
fn exec_source_reports_python_exception() {
    let err = pyxel_pocket::Runtime::new()
        .exec_source("raise RuntimeError('source boom')", "<test>")
        .unwrap_err();

    assert!(err.contains("Traceback (most recent call last):"));
    assert!(err.contains("RuntimeError: source boom"));
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
fn exec_source_accepts_float_modulo() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
assert 5.5 % 2 == 1.5
assert 5 % 2.0 == 1.0
assert -1.0 % 2 == 1.0
",
            "<test>",
        )
        .unwrap();
}

#[test]
fn exec_source_matches_cpython_math_floor_and_ceil_types() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import math

assert math.floor(3.9) == 3
assert math.floor(-3.1) == -4
assert isinstance(math.floor(3.9), int)
assert math.ceil(3.1) == 4
assert math.ceil(-3.9) == -3
assert isinstance(math.ceil(3.1), int)
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
fn audio_play_accepts_sound_instances() {
    let script = unique_temp_path("pyxel-pocket-play-sound-instances", "py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(8, 8, headless=True)

sound0 = pyxel.Sound()
sound0.set('c0d0', 'ss', '77', 'nn', 8)
sound1 = pyxel.Sound()
sound1.set('e0f0', 'pp', '66', 'ff', 8)
pyxel.sounds[0] = sound0
pyxel.sounds[1] = sound1

pyxel.play(0, [0, 1], loop=True)
assert pyxel.play_pos(0) == None or pyxel.play_pos(0)[0] == 0
pyxel.play(0, sound0)
assert pyxel.play_pos(0) == None or pyxel.play_pos(0)[0] == 0
pyxel.play(0, [sound0, sound1], loop=True)
assert pyxel.play_pos(0) == None or pyxel.play_pos(0)[0] == 0
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
fn reset_restarts_current_script_once() {
    let marker = unique_temp_path("pyxel-pocket-reset-marker", "txt");
    let script = unique_temp_path("pyxel-pocket-reset", "py");
    fs::write(
        &script,
        format!(
            "\
import os
import pyxel

marker = '{}'
if os.path.exists(marker):
    with open(marker, 'a') as file:
        file.write('child\\n')
else:
    with open(marker, 'w') as file:
        file.write('parent\\n')
    pyxel.init(8, 8, headless=True)
    pyxel.reset()
",
            escape_python_path(&marker.to_string_lossy()),
        ),
    )
    .unwrap();

    let output = run_with_timeout(
        Command::new(env!("CARGO_BIN_EXE_pyxel-pocket")).arg(&script),
        Duration::from_secs(10),
    )
    .unwrap();

    let marker_contents = fs::read_to_string(&marker).unwrap_or_default();
    let _ = fs::remove_file(script);
    let _ = fs::remove_file(marker);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(marker_contents, "parent\nchild\n");
}

#[test]
fn os_environ_updates_are_inherited_after_reset() {
    let marker = unique_temp_path("pyxel-pocket-env-reset-marker", "txt");
    let script = unique_temp_path("pyxel-pocket-env-reset", "py");
    fs::write(
        &script,
        format!(
            "\
import os
import pyxel

marker = '{}'
if os.path.exists(marker):
    with open(marker, 'a') as file:
        file.write(os.environ.get('PYXEL_POCKET_ENV_WRITE', 'missing') + '\\n')
        file.write(os.environ.get('PYXEL_POCKET_ENV_READ', 'removed') + '\\n')
else:
    assert os.environ['PYXEL_POCKET_ENV_READ'] == 'from-parent'
    with open(marker, 'w') as file:
        file.write('parent\\n')
    os.environ['PYXEL_POCKET_ENV_WRITE'] = 'from-script'
    os.environ.pop('PYXEL_POCKET_ENV_READ', None)
    pyxel.init(8, 8, headless=True)
    pyxel.reset()
",
            escape_python_path(&marker.to_string_lossy()),
        ),
    )
    .unwrap();

    let output = run_with_timeout(
        Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
            .arg(&script)
            .env("PYXEL_POCKET_ENV_READ", "from-parent"),
        Duration::from_secs(10),
    )
    .unwrap();

    let marker_contents = fs::read_to_string(&marker).unwrap_or_default();
    let _ = fs::remove_file(script);
    let _ = fs::remove_file(marker);

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(marker_contents, "parent\nfrom-script\nremoved\n");
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
assert os.getenv('PYXEL_POCKET_VALUE') == 'ok'
assert os.environ.pop('PYXEL_POCKET_VALUE', None) == 'ok'
raised = False
try:
    os.environ.pop('PYXEL_POCKET_MISSING')
except KeyError:
    raised = True
if not raised:
    raise AssertionError('missing pop did not raise')
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
fn exec_source_matches_cpython_random_seed_zero() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import random

random.seed(0)
assert [random.choice([-1, 1]) for _ in range(8)] == [1, 1, -1, 1, 1, 1, 1, 1]
random.seed(0)
assert [random.randint(-3, 3) for _ in range(8)] == [3, 0, 3, 0, -3, -1, 1, 0]
random.seed(0)
assert random.sample(range(100), 10) == [49, 97, 53, 5, 33, 65, 62, 51, 38, 61]
random.seed(0)
assert random.sample(range(10), 5) == [6, 9, 0, 2, 4]
random.seed(0)
assert abs(random.random() - 0.8444218515250481) < 0.000000000000001
assert abs(random.random() - 0.7579544029403025) < 0.000000000000001
assert abs(random.random() - 0.420571580830845) < 0.000000000000001
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
fn exec_source_accepts_int_enum_arithmetic() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
from enum import IntEnum, auto

class StageNum(IntEnum):
    STAGE_1 = auto()
    STAGE_2 = auto()
    STAGE_3 = auto()

assert StageNum.STAGE_2 % 2 == 0
assert StageNum.STAGE_1 + 1 == 2
assert StageNum.STAGE_1 < StageNum.STAGE_3
assert StageNum(StageNum.STAGE_1 + 1) == StageNum.STAGE_2
lookup = {StageNum.STAGE_2: 'two'}
assert lookup[StageNum(StageNum.STAGE_1 + 1)] == 'two'
assert str(StageNum.STAGE_1) == '1'
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
fn run_reports_update_callback_exception() {
    let script = unique_temp_path("pyxel-pocket-run-callback-error", "py");
    fs::write(
        &script,
        "\
import pyxel

pyxel.init(16, 16, headless=True)

def update():
    raise RuntimeError('callback boom')

def draw():
    pyxel.cls(pyxel.COLOR_BLACK)

pyxel.run(update, draw)
",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    let _ = fs::remove_file(script);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "stderr:\n{stderr}");
    assert!(stderr.contains("Traceback (most recent call last):"));
    assert!(stderr.contains("RuntimeError: callback boom"));
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

struct CaptureStep<'a> {
    frame: u32,
    presses: &'a [&'a str],
    mouse: Option<(i32, i32)>,
    capture: bool,
}

impl<'a> CaptureStep<'a> {
    fn capture(frame: u32) -> Self {
        Self {
            frame,
            presses: &[],
            mouse: None,
            capture: true,
        }
    }

    fn capture_with_presses(frame: u32, presses: &'a [&'a str]) -> Self {
        Self {
            frame,
            presses,
            mouse: None,
            capture: true,
        }
    }

    fn capture_with_mouse_and_presses(
        frame: u32,
        mouse: (i32, i32),
        presses: &'a [&'a str],
    ) -> Self {
        Self {
            frame,
            presses,
            mouse: Some(mouse),
            capture: true,
        }
    }

    fn skip_with_presses(frame: u32, presses: &'a [&'a str]) -> Self {
        Self {
            frame,
            presses,
            mouse: None,
            capture: false,
        }
    }
}

fn assert_example_reference_screenshots(name: &str, steps: &[CaptureStep<'_>]) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let example = root.join(format!("python/pyxel/examples/{name}.py"));
    let refs_dir = root.join("python/tests/references/examples");
    let script = unique_temp_path(&format!("pyxel-pocket-capture-{name}"), "py");
    let actuals = steps
        .iter()
        .filter(|step| step.capture)
        .map(|step| {
            (
                step.frame,
                unique_temp_path(
                    &format!("pyxel-pocket-capture-{name}-f{}", step.frame),
                    "png",
                ),
            )
        })
        .collect::<Vec<_>>();

    let mut source = format!(
        r#"import os
import pyxel

__captured = {{}}
__original_init = pyxel.init

def __patched_init(*args, **kwargs):
    kwargs['headless'] = True
    kwargs['fps'] = 1000000
    __original_init(*args, **kwargs)
    os.chdir('{}')
    pyxel.rseed(0)
    pyxel.nseed(0)

def __patched_run(update, draw):
    __captured['update'] = update
    __captured['draw'] = draw

def __patched_show():
    pass

pyxel.init = __patched_init
pyxel.run = __patched_run
pyxel.show = __patched_show
os.chdir('{}')
__file__ = '{}'
"#,
        escape_python_path(&example.parent().unwrap().to_string_lossy()),
        escape_python_path(&example.parent().unwrap().to_string_lossy()),
        escape_python_path(&example.to_string_lossy()),
    );
    source.push_str(&fs::read_to_string(&example).unwrap());
    source.push_str("\n__current_frame = 0\n");
    let mut actual_index = 0usize;
    for step in steps {
        let actual = if step.capture {
            let path = &actuals[actual_index].1;
            actual_index += 1;
            Some(path.as_path())
        } else {
            None
        };
        append_capture_step_optional(&mut source, step, actual);
    }
    fs::write(&script, source).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for (frame, actual) in &actuals {
        let expected = refs_dir.join(format!("{name}_f{frame}.png"));
        assert_capture_matches_reference(actual, &expected);
    }

    let _ = fs::remove_file(script);
    for (_, actual) in actuals {
        let _ = fs::remove_file(actual);
    }
}

fn assert_capture_matches_reference(actual: &std::path::Path, expected: &std::path::Path) {
    let actual_bytes =
        fs::read(actual).unwrap_or_else(|err| panic!("failed to read {}: {err}", actual.display()));
    let expected_bytes = fs::read(expected)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", expected.display()));
    let actual_image = image::load_from_memory(&actual_bytes)
        .unwrap_or_else(|err| panic!("failed to decode {}: {err}", actual.display()))
        .to_rgba8();
    let expected_image = image::load_from_memory(&expected_bytes)
        .unwrap_or_else(|err| panic!("failed to decode {}: {err}", expected.display()))
        .to_rgba8();
    assert_eq!(
        actual_image.dimensions(),
        expected_image.dimensions(),
        "PocketPy capture size differed from {} (actual: {}; expected: {})",
        expected.display(),
        actual.display(),
        expected.display(),
    );
    assert!(
        actual_image.as_raw() == expected_image.as_raw(),
        "PocketPy capture pixels differed from {} (actual: {}, {} bytes; expected: {} bytes)",
        expected.display(),
        actual.display(),
        actual_bytes.len(),
        expected_bytes.len()
    );
}

fn assert_flip_example_reference_screenshots(name: &str, steps: &[CaptureStep<'_>]) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let example = root.join(format!("python/pyxel/examples/{name}.py"));
    let refs_dir = root.join("python/tests/references/examples");
    let script = unique_temp_path(&format!("pyxel-pocket-flip-capture-{name}"), "py");
    let actuals = steps
        .iter()
        .filter(|step| step.capture)
        .map(|step| {
            (
                step.frame,
                unique_temp_path(
                    &format!("pyxel-pocket-flip-capture-{name}-f{}", step.frame),
                    "png",
                ),
            )
        })
        .collect::<Vec<_>>();
    let max_frame = steps.iter().map(|step| step.frame).max().unwrap_or(0);

    let mut source = format!(
        r#"import os
import pyxel

class __FlipCapture(Exception):
    pass

__frame_count = 0
__original_init = pyxel.init
__original_flip = pyxel.flip

def __patched_init(*args, **kwargs):
    kwargs['headless'] = True
    kwargs['fps'] = 1000000
    __original_init(*args, **kwargs)
    os.chdir('{}')
    pyxel.rseed(0)
    pyxel.nseed(0)

def __patched_flip():
    global __frame_count
    __original_flip()
    __frame_count += 1
"#,
        escape_python_path(&example.parent().unwrap().to_string_lossy()),
    );
    for (frame, actual) in &actuals {
        source.push_str(&format!(
            "    if __frame_count == {}:\n        pyxel.screenshot('{}')\n",
            frame,
            escape_python_path(&actual.to_string_lossy()),
        ));
    }
    source.push_str(&format!(
        "\n    if __frame_count >= {}:\n        raise __FlipCapture()\n\npyxel.init = __patched_init\npyxel.flip = __patched_flip\nos.chdir('{}')\n__file__ = '{}'\ntry:\n",
        max_frame,
        escape_python_path(&example.parent().unwrap().to_string_lossy()),
        escape_python_path(&example.to_string_lossy()),
    ));
    for line in fs::read_to_string(&example).unwrap().lines() {
        source.push_str("    ");
        source.push_str(line);
        source.push('\n');
    }
    source.push_str("except __FlipCapture:\n    pass\n");
    fs::write(&script, source).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for (frame, actual) in &actuals {
        let expected = refs_dir.join(format!("{name}_f{frame}.png"));
        assert_capture_matches_reference(actual, &expected);
    }

    let _ = fs::remove_file(script);
    for (_, actual) in actuals {
        let _ = fs::remove_file(actual);
    }
}

fn assert_app_reference_screenshots(name: &str, steps: &[CaptureStep<'_>]) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let pyxapp = root.join(format!("python/pyxel/examples/apps/{name}.pyxapp"));
    let refs_dir = root.join("python/tests/references/apps");
    let script = unique_temp_path(&format!("pyxel-pocket-capture-{name}"), "py");
    let actuals = steps
        .iter()
        .filter(|step| step.capture)
        .map(|step| {
            (
                step.frame,
                unique_temp_path(
                    &format!("pyxel-pocket-capture-{name}-f{}", step.frame),
                    "png",
                ),
            )
        })
        .collect::<Vec<_>>();

    let mut source = format!(
        r#"import os
import pyxel
import pyxel.cli
import random

__captured = {{}}
__original_init = pyxel.init

def __patched_init(*args, **kwargs):
    kwargs['headless'] = True
    kwargs['fps'] = 1000000
    __original_init(*args, **kwargs)
    pyxel.rseed(0)
    pyxel.nseed(0)
    random.seed(0)

def __patched_run(update, draw):
    __captured['update'] = update
    __captured['draw'] = draw

def __patched_show():
    pass

pyxel.init = __patched_init
pyxel.run = __patched_run
pyxel.show = __patched_show
pyxel.cli.play_pyxel_app('{}')
__current_frame = 0
"#,
        escape_python_path(&pyxapp.to_string_lossy()),
    );
    let mut actual_index = 0usize;
    for step in steps {
        let actual = if step.capture {
            let path = &actuals[actual_index].1;
            actual_index += 1;
            Some(path.as_path())
        } else {
            None
        };
        append_capture_step_optional(&mut source, step, actual);
    }
    fs::write(&script, source).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .arg(&script)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for (frame, actual) in &actuals {
        let expected = refs_dir.join(format!("{name}_f{frame}.png"));
        assert_capture_matches_reference(actual, &expected);
    }

    let _ = fs::remove_file(script);
    for (_, actual) in actuals {
        let _ = fs::remove_file(actual);
    }
}

fn append_capture_step_optional(
    source: &mut String,
    step: &CaptureStep<'_>,
    actual: Option<&std::path::Path>,
) {
    if let Some((x, y)) = step.mouse {
        source.push_str(&format!("pyxel.set_mouse_pos({}, {})\n", x, y));
    }
    for key in step.presses {
        source.push_str(&format!("pyxel.set_btn({key}, True)\n"));
    }
    source.push_str(&format!(
        "\
__target_frame = {}
while __current_frame < __target_frame:
    __captured['update']()
    __captured['draw']()
    pyxel.flip()
    __current_frame += 1
",
        step.frame,
    ));
    if let Some(actual) = actual {
        source.push_str(&format!(
            "pyxel.screenshot('{}')\n",
            escape_python_path(&actual.to_string_lossy()),
        ));
    }
    for key in step.presses {
        source.push_str(&format!("pyxel.set_btn({key}, False)\n"));
    }
}

#[test]
fn hello_example_matches_reference_screenshot() {
    assert_example_reference_screenshots("01_hello_pyxel", &[CaptureStep::capture(8)]);
}

#[test]
fn draw_api_example_matches_reference_screenshots() {
    assert_example_reference_screenshots(
        "03_draw_api",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_presses(155, &["pyxel.KEY_SPACE"]),
        ],
    );
}

#[test]
fn click_game_example_matches_reference_screenshots() {
    assert_example_reference_screenshots(
        "06_click_game",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_mouse_and_presses(
                10,
                (110, 146),
                &["pyxel.MOUSE_BUTTON_LEFT"],
            ),
        ],
    );
}

#[test]
fn shipped_run_examples_match_reference_screenshots() {
    assert_example_reference_screenshots("01_hello_pyxel", &[CaptureStep::capture(8)]);
    assert_example_reference_screenshots("02_jump_game", &[CaptureStep::capture(10)]);
    assert_example_reference_screenshots(
        "03_draw_api",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_presses(155, &["pyxel.KEY_SPACE"]),
        ],
    );
    assert_example_reference_screenshots("04_sound_api", &[CaptureStep::capture(1)]);
    assert_example_reference_screenshots("05_color_palette", &[CaptureStep::capture(0)]);
    assert_example_reference_screenshots(
        "06_click_game",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_mouse_and_presses(
                10,
                (110, 146),
                &["pyxel.MOUSE_BUTTON_LEFT"],
            ),
        ],
    );
    assert_example_reference_screenshots("07_snake", &[CaptureStep::capture(1)]);
    assert_example_reference_screenshots(
        "08_triangle_api",
        &[CaptureStep::capture(1), CaptureStep::capture(200)],
    );
    assert_example_reference_screenshots(
        "09_shooter",
        &[
            CaptureStep::capture_with_presses(1, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(120),
        ],
    );

    let mut platformer_steps = vec![CaptureStep::capture(1)];
    for frame in (2..80).step_by(2) {
        platformer_steps.push(CaptureStep::skip_with_presses(
            frame,
            &["pyxel.KEY_RIGHT", "pyxel.KEY_SPACE"],
        ));
    }
    platformer_steps.push(CaptureStep::capture(80));
    assert_example_reference_screenshots("10_platformer", &platformer_steps);

    assert_example_reference_screenshots(
        "11_offscreen",
        &[CaptureStep::capture(1), CaptureStep::capture(121)],
    );
    assert_example_reference_screenshots(
        "12_perlin_noise",
        &[CaptureStep::capture(1), CaptureStep::capture(40)],
    );
    assert_example_reference_screenshots("13_custom_font", &[CaptureStep::capture(0)]);
    assert_example_reference_screenshots("14_synthesizer", &[CaptureStep::capture(1)]);
    assert_example_reference_screenshots("15_tiled_map_file", &[CaptureStep::capture(1)]);
    assert_example_reference_screenshots(
        "16_transform",
        &[CaptureStep::capture(1), CaptureStep::capture(45)],
    );
    assert_example_reference_screenshots("17_app_launcher", &[CaptureStep::capture(1)]);
    assert_example_reference_screenshots(
        "18_audio_playback",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_presses(3, &["pyxel.KEY_RETURN"]),
        ],
    );
    assert_example_reference_screenshots(
        "19_perspective",
        &[
            CaptureStep::capture(1),
            CaptureStep::capture_with_presses(20, &["pyxel.KEY_RIGHT", "pyxel.KEY_W"]),
        ],
    );
}

#[test]
fn flip_animation_example_matches_reference_screenshots() {
    assert_flip_example_reference_screenshots(
        "99_flip_animation",
        &[CaptureStep::capture(1), CaptureStep::capture(30)],
    );
}

#[test]
fn megaball_app_matches_reference_screenshots() {
    assert_app_reference_screenshots(
        "megaball",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_RETURN"]),
            CaptureStep::skip_with_presses(35, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(90),
        ],
    );
}

#[test]
fn shipped_apps_match_reference_screenshots() {
    assert_app_reference_screenshots(
        "megaball",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_RETURN"]),
            CaptureStep::skip_with_presses(35, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(90),
        ],
    );
    assert_app_reference_screenshots(
        "mega_wing",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(150),
        ],
    );
    assert_app_reference_screenshots(
        "space_rescue",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(60),
        ],
    );
    assert_app_reference_screenshots(
        "cursed_caverns",
        &[
            CaptureStep::capture(70),
            CaptureStep::skip_with_presses(71, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(100),
        ],
    );
    assert_app_reference_screenshots(
        "30sec_of_daylight",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(60),
        ],
    );
    assert_app_reference_screenshots(
        "laser-jetman",
        &[
            CaptureStep::capture(210),
            CaptureStep::skip_with_presses(211, &["pyxel.KEY_RETURN"]),
            CaptureStep::capture(270),
        ],
    );
    assert_app_reference_screenshots(
        "vortexion",
        &[
            CaptureStep::capture(30),
            CaptureStep::skip_with_presses(31, &["pyxel.KEY_Z"]),
            CaptureStep::capture(200),
        ],
    );
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
