from pathlib import Path

import pyxel

REFS_DIR = Path(__file__).parent / "references" / "audio"
ASSETS_DIR = Path(__file__).parent.parent / "pyxel" / "examples" / "assets"

# Sound and music data quoted from 04_sound_api.py
CLASSIC_SOUNDS = [
    (
        "e2e2c2g1 g1g1c2e2 d2d2d2g2 g2g2rr c2c2a1e1 e1e1a1c2 b1b1b1e2 e2e2rr",
        "p",
        "6",
        "vffn fnff vffs vfnn",
        25,
    ),
    (
        "r a1b1c2 b1b1c2d2 g2g2g2g2 c2c2d2e2 f2f2f2e2 f2e2d2c2 d2d2d2d2 g2g2r r ",
        "s",
        "6",
        "nnff vfff vvvv vfff svff vfff vvvv svnn",
        25,
    ),
    (
        "c1g1c1g1 c1g1c1g1 b0g1b0g1 b0g1b0g1 a0e1a0e1 a0e1a0e1 g0d1g0d1 g0d1g0d1",
        "t",
        "7",
        "n",
        25,
    ),
    (
        "f0c1f0c1 g0d1g0d1 c1g1c1g1 a0e1a0e1 f0c1f0c1 f0c1f0c1 g0d1g0d1 g0d1g0d1",
        "t",
        "7",
        "n",
        25,
    ),
    ("f0ra4r f0ra4r f0ra4r f0f0a4r", "n", "6622 6622 6622 6422", "f", 25),
]

# Every classic effect on short notes verifies sub-tick timing
EFFECT_SWEEP_SOUND = (
    "c2d2e2f2g2a2 c2d2e2f2g2a2",
    "s",
    "7",
    "nsvfhq nsvfhq",
    4,
)

MML_MODULATION = (
    "T120 O4 L16 @VIB1{0,12,100} @VIB1 C D @VIB2{6,12,100} @VIB2 E F @VIB0 "
    "@GLI1{-200,12} @GLI1 G A @GLI2{*,*} @GLI2 B >C< T90 @VIB1 C D"
)

# Title music MML quoted from 09_shooter.py
MML_PARTS = [
    "T128 Q96 @2 @ENV1{127,6,96} O4 L16 @VIB1{36,18,25} K-2"
    "D8.C8.D4G8AB->CD C8.<F2R FFGA B-8.A8.B-4.GGAB-"
    "RR>CC<B->C8 D8.D8CD8.<",
    "T128 Q90 @0 V96 O3 L16"
    "FFR4 FFR4 <F4> E-E-R4 E-E-R4 <E-4> D-D-R4 D-D-R4 <D-4> E-E-R4 E-E-R4 EEE8",
    "T128 Q50 @3 L16 @ENV1{48,8,0} @ENV2{127,6,0}"
    "[@ENV1 O7 FFR4 FFR4 @ENV2 O3 G4]3 @ENV1 O7 FFR4 FFR4 FF @ENV2 O3 G8",
]


def _compare_or_update(name, rendered_path, update_references):
    ref_path = REFS_DIR / f"{name}.wav"
    rendered = Path(rendered_path).read_bytes()
    if update_references:
        REFS_DIR.mkdir(parents=True, exist_ok=True)
        ref_path.write_bytes(rendered)
    else:
        assert rendered == ref_path.read_bytes(), f"{name}.wav changed"


def _append_sounds(sounds):
    base_index = len(pyxel.sounds)
    for sound in sounds:
        pyxel.sounds.append(sound)
    return base_index


def _pop_sounds(count):
    for _ in range(count):
        pyxel.sounds.pop()


class TestAudioRender:
    def test_classic_sound(self, tmp_path, update_references):
        snd = pyxel.Sound()
        snd.set(*CLASSIC_SOUNDS[0])
        path = tmp_path / "out.wav"
        snd.save(str(path), 1.5)
        _compare_or_update("classic_sound", path, update_references)

    def test_mml_sound(self, tmp_path, update_references):
        snd = pyxel.Sound()
        snd.mml(MML_PARTS[0])
        path = tmp_path / "out.wav"
        snd.save(str(path), 1.5)
        _compare_or_update("mml_sound", path, update_references)

    def test_classic_music(self, tmp_path, update_references):
        sounds = []
        for params in CLASSIC_SOUNDS:
            snd = pyxel.Sound()
            snd.set(*params)
            sounds.append(snd)
        base = _append_sounds(sounds)
        try:
            msc = pyxel.Music()
            msc.set([base, base + 1], [base + 2, base + 3], [base + 4])
            path = tmp_path / "out.wav"
            msc.save(str(path), 2.0)
        finally:
            _pop_sounds(len(sounds))
        _compare_or_update("classic_music", path, update_references)

    def test_mml_music(self, tmp_path, update_references):
        sounds = []
        for mml in MML_PARTS:
            snd = pyxel.Sound()
            snd.mml(mml)
            sounds.append(snd)
        base = _append_sounds(sounds)
        try:
            msc = pyxel.Music()
            msc.set([base], [base + 1], [base + 2])
            path = tmp_path / "out.wav"
            msc.save(str(path), 2.0)
        finally:
            _pop_sounds(len(sounds))
        _compare_or_update("mml_music", path, update_references)

    def test_classic_effects(self, tmp_path, update_references):
        snd = pyxel.Sound()
        snd.set(*EFFECT_SWEEP_SOUND)
        path = tmp_path / "out.wav"
        snd.save(str(path), 0.5)
        _compare_or_update("classic_effects", path, update_references)

    def test_mml_modulation(self, tmp_path, update_references):
        snd = pyxel.Sound()
        snd.mml(MML_MODULATION)
        path = tmp_path / "out.wav"
        snd.save(str(path), 2.0)
        _compare_or_update("mml_modulation", path, update_references)

    def test_pcm_sound(self, tmp_path, update_references):
        snd = pyxel.Sound()
        snd.pcm(str(ASSETS_DIR / "audio_bgm1.ogg"))
        path = tmp_path / "out.wav"
        snd.save(str(path), 0.5)
        _compare_or_update("pcm_sound", path, update_references)
