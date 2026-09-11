import os
import shutil
import subprocess
import sys

import pytest
import pyxel


class TestPlay:
    @pytest.mark.parametrize("form", ["index", "indices", "sound", "sounds", "mml"])
    def test_play_sound_forms(self, form):
        snd = pyxel.Sound()
        snd.set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        sounds = {
            "index": 0,
            "indices": [0, 0],
            "sound": snd,
            "sounds": [snd, snd],
            "mml": "T120 O4 L4 CDEF",
        }

        pyxel.stop(3)
        try:
            pyxel.play(3, sounds[form], loop=True)
            pos = pyxel.play_pos(3)
            assert isinstance(pos, tuple)
            assert len(pos) == 2
        finally:
            pyxel.stop(3)
        assert pyxel.play_pos(3) is None

    @pytest.mark.parametrize("resume", [False, True])
    def test_play_seek_past_interruption(self, resume):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, loop=True)
        try:
            pyxel.play(3, "T120 L4 C", sec=1, resume=resume)
            assert (pyxel.play_pos(3) is not None) is resume
        finally:
            pyxel.stop(3)

    @pytest.mark.parametrize("options", [{}, {"tick": None}])
    def test_play_seek_past_end_stops(self, options):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, loop=True)
        try:
            pyxel.play(3, 0, sec=1, **options)
            assert pyxel.play_pos(3) is None
        finally:
            pyxel.stop(3)

    def test_play_with_tick_deprecated(self, capfd):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.play(3, 0, sec=0, tick=120)  # type: ignore[call-arg]
        out = capfd.readouterr().out
        assert (
            out
            == "tick option of pyxel.play is deprecated. Use sec in seconds (tick / 120) instead.\n"
        )
        assert pyxel.play_pos(3) is None
        pyxel.stop(3)


class TestPlaym:
    def test_playm_starts_channels_and_stop_clears_them(self):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0], [0])
        pyxel.stop()
        try:
            pyxel.playm(0, loop=True)
            assert pyxel.play_pos(0) is not None
            assert pyxel.play_pos(1) is not None
        finally:
            pyxel.stop()
        assert pyxel.play_pos(0) is None
        assert pyxel.play_pos(1) is None

    @pytest.mark.parametrize("options", [{}, {"tick": None}])
    def test_playm_seek_past_end_stops(self, options):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0])
        pyxel.playm(0, loop=True)
        try:
            pyxel.playm(0, sec=1, **options)
            assert pyxel.play_pos(0) is None
        finally:
            pyxel.stop()

    def test_playm_with_tick_deprecated(self, capfd):
        pyxel.sounds[0].set("c2e2g2", "sss", "777", "nnn", 10)
        pyxel.musics[0].set([0])
        pyxel.playm(0, sec=0, tick=240)  # type: ignore[call-arg]
        out = capfd.readouterr().out
        assert (
            out
            == "tick option of pyxel.playm is deprecated. Use sec in seconds (tick / 120) instead.\n"
        )
        assert pyxel.play_pos(0) is None
        pyxel.stop()


class TestPlayPos:
    def test_play_pos_when_not_playing(self):
        pyxel.stop(3)
        result = pyxel.play_pos(3)
        assert result is None


class TestGenBgm:
    def test_basic(self):
        result = pyxel.gen_bgm(0, 0, 3, 42)
        assert isinstance(result, list)
        assert len(result) == 4
        assert all(isinstance(s, str) for s in result)
        assert len(result[0]) > 0

    def test_seed_reproducible(self):
        result1 = pyxel.gen_bgm(0, 0, 3, 42)
        result2 = pyxel.gen_bgm(0, 0, 3, 42)
        assert result1 == result2

    def test_different_seeds_differ(self):
        result1 = pyxel.gen_bgm(0, 0, 3, 1)
        result2 = pyxel.gen_bgm(0, 0, 3, 2)
        assert result1 != result2

    def test_all_presets(self):
        for preset in range(8):
            result = pyxel.gen_bgm(preset, 0, 0, 1)
            assert isinstance(result, list)
            assert len(result) == 4

    def test_all_instrumentations(self):
        for instr in range(4):
            result = pyxel.gen_bgm(0, 0, instr, 1)
            assert isinstance(result, list)
            assert len(result) == 4

    def test_transpose_changes_output(self):
        result_default = pyxel.gen_bgm(0, 0, 3, 42)
        result_transposed = pyxel.gen_bgm(0, 3, 3, 42)
        assert result_default != result_transposed

    def test_instr_changes_output(self):
        result_default = pyxel.gen_bgm(0, 0, 3, 42)
        result_other_instr = pyxel.gen_bgm(0, 0, 0, 42)
        assert result_default != result_other_instr

    def test_play_and_stop(self):
        # A callback lock inversion must time out a child, not hang the test suite.
        code = """
import pyxel

pyxel.init(16, 16, headless=True)
pyxel.sounds[0].set("c2e2g2", "s", "7", "n", 10)
pyxel.musics[0].set([0])

for seed in range(16):
    expected = pyxel.gen_bgm(0, 0, 3, seed)
    assert pyxel.gen_bgm(0, 0, 3, seed, play=True) == expected
    pyxel.stop()
    pyxel.playm(0, loop=True)
    pyxel.stop()
"""

        result = subprocess.run(
            [sys.executable, "-B", "-c", code],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
            env={**os.environ, "SDL_AUDIODRIVER": "dummy"},
        )
        assert result.returncode == 0, result.stdout + result.stderr
        assert "Failed to initialize audio device" not in result.stdout


class TestAudioExport:
    def test_mp4_export_preserves_existing_temporary_file(self, tmp_path):
        if shutil.which("ffmpeg") is None:
            pytest.skip("FFmpeg is required for MP4 export")
        sentinel = tmp_path / "pyxel_mp4_image.png"
        sentinel.write_bytes(b"existing user file")
        code = """
import sys
from pathlib import Path

import pyxel

root = Path(sys.argv[1])
pyxel.init(8, 8, headless=True)
pyxel.sounds[0].set("c2", "t", "7", "n", 10)
music = pyxel.Music()
music.set([0])
for name, source in [("sound", pyxel.sounds[0]), ("music", music)]:
    source.save(str(root / name), 0.1, ffmpeg=True)
    assert (root / f"{name}.wav").read_bytes()[:4] == b"RIFF"
    assert (root / f"{name}.mp4").read_bytes()[4:8] == b"ftyp"
"""
        result = subprocess.run(
            [sys.executable, "-B", "-c", code, str(tmp_path)],
            capture_output=True,
            text=True,
            timeout=30,
            check=False,
            env={
                **os.environ,
                "SDL_AUDIODRIVER": "dummy",
                "TMPDIR": str(tmp_path),
                "TEMP": str(tmp_path),
                "TMP": str(tmp_path),
            },
        )
        assert result.returncode == 0, result.stdout + result.stderr
        assert sentinel.read_bytes() == b"existing user file"


class TestDeprecatedAccessors:
    def test_channel_function_aliases_bank_entry(self, capfd):
        result = pyxel.channel(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Channel)

        original = pyxel.channels[0].gain
        try:
            pyxel.channels[0].gain = 0.125
            result.gain = 0.375
            assert pyxel.channels[0].gain == 0.375
        finally:
            pyxel.channels[0].gain = original

        out = capfd.readouterr().out
        assert (
            out == "pyxel.channel(ch) is deprecated. Use pyxel.channels[ch] instead.\n"
        )

    def test_sound_function_aliases_bank_entry(self, capfd):
        result = pyxel.sound(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Sound)

        original = pyxel.sounds[0].speed
        try:
            pyxel.sounds[0].speed = 30
            result.speed = 17
            assert pyxel.sounds[0].speed == 17
        finally:
            pyxel.sounds[0].speed = original

        out = capfd.readouterr().out
        assert out == "pyxel.sound(snd) is deprecated. Use pyxel.sounds[snd] instead.\n"

    def test_music_function_aliases_bank_entry(self, capfd):
        result = pyxel.music(0)  # type: ignore[attr-defined]
        assert isinstance(result, pyxel.Music)

        original = [list(seq) for seq in pyxel.musics[0].seqs]
        try:
            pyxel.musics[0].seqs.clear()
            result.seqs[:] = [[3, 4]]
            assert [list(seq) for seq in pyxel.musics[0].seqs] == [[3, 4]]
        finally:
            pyxel.musics[0].seqs[:] = original

        out = capfd.readouterr().out
        assert out == "pyxel.music(msc) is deprecated. Use pyxel.musics[msc] instead.\n"
