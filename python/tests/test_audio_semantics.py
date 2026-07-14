from contextlib import contextmanager
import math
import struct
import wave

import pytest
import pyxel

RATE = 22050


@pytest.fixture(autouse=True)
def isolate_audio_resources():
    pyxel.stop()
    original_sounds = list(pyxel.sounds[60:64])
    original_music = pyxel.musics[7]
    pyxel.sounds[60:64] = [pyxel.Sound() for _ in range(4)]
    pyxel.musics[7] = pyxel.Music()
    try:
        yield
    finally:
        pyxel.stop()
        pyxel.sounds[60:64] = original_sounds
        pyxel.musics[7] = original_music


@contextmanager
def _raises_exact(exception_type, message):
    with pytest.raises(exception_type) as exc:
        yield
    assert type(exc.value) is exception_type
    assert str(exc.value) == message


def _write_pcm_wav(path, sec=0.05, freq=440.0):
    num_samples = int(RATE * sec)
    with wave.open(str(path), "w") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(RATE)
        w.writeframes(
            b"".join(
                struct.pack("<h", int(12000 * math.sin(2 * math.pi * freq * i / RATE)))
                for i in range(num_samples)
            )
        )


def _read_samples(path):
    with wave.open(str(path)) as w:
        raw = w.readframes(w.getnframes())
    return struct.unpack(f"<{len(raw) // 2}h", raw)


def _rms(samples, start_sec, end_sec):
    seg = samples[int(RATE * start_sec) : int(RATE * end_sec)]
    return math.sqrt(sum(s * s for s in seg) / len(seg))


def _assert_looping_position(pos, duration_sec):
    assert pos is not None
    assert pos[0] == 0
    # Audio-thread timing may advance playback before play_pos obtains the lock.
    assert 0.0 <= pos[1] < duration_sec


class TestPcmSynthBoundary:
    def test_music_save_pcm_then_synth(self, tmp_path):
        pcm_path = tmp_path / "pcm.wav"
        _write_pcm_wav(pcm_path)
        pyxel.sounds[60].pcm(str(pcm_path))
        pyxel.sounds[61].set("c2", "t", "7", "n", 30)
        pyxel.musics[7].set([60, 61], [], [], [])
        out = tmp_path / "out.wav"
        pyxel.musics[7].save(str(out), 0.4)

        samples = _read_samples(out)
        assert _rms(samples, 0.0, 0.045) > 100, "PCM part is silent"
        assert _rms(samples, 0.06, 0.25) > 100, "synth part after PCM is silent"

    def test_music_save_synth_then_pcm(self, tmp_path):
        pcm_path = tmp_path / "pcm.wav"
        _write_pcm_wav(pcm_path)
        pyxel.sounds[60].pcm(str(pcm_path))
        pyxel.sounds[61].set("c2", "t", "7", "n", 30)
        pyxel.musics[7].set([61, 60], [], [], [])
        out = tmp_path / "out.wav"
        pyxel.musics[7].save(str(out), 0.4)

        samples = _read_samples(out)
        assert _rms(samples, 0.0, 0.24) > 100, "synth part is silent"
        assert _rms(samples, 0.26, 0.295) > 100, "PCM part after synth is silent"


class TestPcmSeek:
    def test_loop_seek_wraps(self, tmp_path):
        pcm_path = tmp_path / "pcm.wav"
        _write_pcm_wav(pcm_path)
        pyxel.sounds[60].pcm(str(pcm_path))
        pyxel.play(3, 60, sec=1.0, loop=True)
        pos = pyxel.play_pos(3)
        pyxel.stop(3)

        _assert_looping_position(pos, 0.05)

    def test_seek_across_pcm_lands_in_following_sound(self, tmp_path):
        pcm_path = tmp_path / "pcm.wav"
        _write_pcm_wav(pcm_path)
        pyxel.sounds[60].pcm(str(pcm_path))
        pyxel.sounds[61].set("c2", "t", "7", "n", 12)
        pyxel.sounds[62].set("e2", "t", "7", "n", 12)
        pyxel.play(3, [61, 60, 62], sec=0.2)
        pos = pyxel.play_pos(3)
        pyxel.stop(3)
        assert pos is not None
        assert pos[0] == 2, f"seek landed at {pos} instead of the last sound"


class TestExtremeInputs:
    def test_high_tempo_renders(self, tmp_path):
        snd = pyxel.Sound()
        snd.mml("T5000000 C")
        snd.save(str(tmp_path / "out.wav"), 0.1)

    def test_long_wavetable_renders(self, tmp_path):
        original = list(pyxel.tones[0].wavetable)
        try:
            pyxel.tones[0].wavetable[:] = [8, 0] * 8192
            pyxel.sounds[63].set("c2", "t", "7", "n", 30)
            pyxel.sounds[63].save(str(tmp_path / "out.wav"), 0.1)
        finally:
            pyxel.tones[0].wavetable[:] = original

    def test_long_mml_total_sec(self):
        snd = pyxel.Sound()
        snd.mml("T1 L1 C&C&C&C&C&C&C&C&C&C")
        assert snd.total_sec() == 2399.999755859375

    def test_long_mml_note_renders_audio(self, tmp_path):
        snd = pyxel.Sound()
        snd.mml("T1 L1 C&C&C&C&C&C&C&C&C&C")
        out = tmp_path / "out.wav"

        snd.save(str(out), 0.1)

        assert any(_read_samples(out))

    def test_long_seek_position(self):
        pyxel.sounds[63].set("c2c2c2c2c2c2c2c2", "t", "7", "n", 15)
        pyxel.play(3, 63, sec=2400.5, loop=True)
        pos = pyxel.play_pos(3)
        pyxel.stop(3)
        clock_rate = 1789773
        loop_clocks = 8 * 15 * (clock_rate // 120)
        _assert_looping_position(pos, loop_clocks / clock_rate)


class TestInvalidInputs:
    def test_speed_zero_raises(self):
        with _raises_exact(ValueError, "speed must be greater than 0"):
            pyxel.sounds[63].speed = 0
        with _raises_exact(ValueError, "speed must be greater than 0"):
            pyxel.sounds[63].set("c2", "t", "7", "n", 0)

    def test_sub_sample_duration_raises(self, tmp_path):
        sound_path = tmp_path / "sound.wav"
        music_path = tmp_path / "music.wav"
        message = "duration_sec is too short to produce an audio sample"

        with _raises_exact(Exception, message):
            pyxel.sounds[63].save(str(sound_path), 1e-9)
        with _raises_exact(Exception, message):
            pyxel.musics[7].save(str(music_path), 1e-9)

        assert not sound_path.exists()
        assert not music_path.exists()

    def test_sample_bits_out_of_range_raises(self):
        for bits in [0, 17]:
            with _raises_exact(ValueError, "sample_bits must be between 1 and 16"):
                pyxel.tones[0].sample_bits = bits

    def test_empty_tone_bank_raises(self, tmp_path):
        original_tones = list(pyxel.tones)
        pyxel.sounds[63].set("c2", "t", "7", "n", 30)
        try:
            pyxel.tones.clear()
            message = "tones must not be empty"

            with _raises_exact(Exception, message):
                pyxel.sounds[63].save(str(tmp_path / "out.wav"), 0.1)
            with _raises_exact(ValueError, message):
                pyxel.play(3, 63)
        finally:
            pyxel.tones[:] = original_tones

    def test_empty_tone_bank_rejects_music_atomically(self, tmp_path):
        pcm_path = tmp_path / "pcm.wav"
        _write_pcm_wav(pcm_path)
        pyxel.sounds[60].pcm(str(pcm_path))
        pyxel.sounds[61].set("c2", "t", "7", "n", 30)
        pyxel.musics[7].set([60], [61], [], [])
        original_tones = list(pyxel.tones)
        try:
            pyxel.tones.clear()

            with _raises_exact(Exception, "tones must not be empty"):
                pyxel.playm(7, loop=True)

            assert pyxel.play_pos(0) is None
            assert pyxel.play_pos(1) is None
        finally:
            pyxel.stop()
            pyxel.tones[:] = original_tones

    def test_gen_bgm_handles_empty_tone_bank(self):
        original_tones = list(pyxel.tones)
        try:
            pyxel.tones.clear()

            assert pyxel.gen_bgm(0, 0, 0, 0, play=True)
        finally:
            pyxel.stop()
            pyxel.tones[:] = original_tones

    def test_music_invalid_sound_index_raises(self, tmp_path):
        pyxel.musics[7].set([999999], [], [], [])
        message = "Music contains an invalid sound index"

        with _raises_exact(Exception, message):
            pyxel.musics[7].save(str(tmp_path / "out.wav"), 0.1)
        with _raises_exact(Exception, message):
            pyxel.playm(7)

    def test_non_finite_duration_raises(self, tmp_path):
        pyxel.sounds[63].set("c2", "t", "7", "n", 30)
        with _raises_exact(Exception, "duration_sec must be finite"):
            pyxel.sounds[63].save(str(tmp_path / "out.wav"), float("nan"))
        with _raises_exact(Exception, "duration_sec must be finite"):
            pyxel.musics[7].save(str(tmp_path / "out.wav"), float("nan"))

    def test_invalid_playback_sec_raises(self):
        pyxel.sounds[63].set("c2", "t", "7", "n", 30)
        channel = pyxel.Channel()

        calls = [
            lambda sec: pyxel.play(3, 63, sec=sec, loop=True),
            lambda sec: pyxel.playm(7, sec=sec, loop=True),
            lambda sec: channel.play(pyxel.sounds[63], sec=sec, loop=True),
        ]
        for call in calls:
            with _raises_exact(ValueError, "sec must be finite"):
                call(float("nan"))

        message = "sec must be greater than or equal to 0"
        for call in calls:
            with _raises_exact(ValueError, message):
                call(-1.0)
