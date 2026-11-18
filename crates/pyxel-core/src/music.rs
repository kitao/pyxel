use crate::audio::Audio;
use crate::channel::Channel;
use crate::pyxel;

#[derive(Clone)]
pub struct Music {
    pub seqs: Vec<Vec<u32>>,
}

define_audio_type!(RcMusic, Music);

impl Music {
    pub fn new() -> RcMusic {
        new_audio_type!(Self { seqs: Vec::new() })
    }

    pub fn set(&mut self, seqs: Vec<Vec<u32>>) {
        self.seqs = seqs;
        let num_channels = pyxel::channels().len();
        self.seqs
            .resize_with(self.seqs.len().max(num_channels), Vec::new);
    }

    pub fn save(
        &self,
        filename: &str,
        duration_sec: f32,
        use_ffmpeg: Option<bool>,
    ) -> Result<(), String> {
        let num_samples = Audio::duration_samples(duration_sec)?;
        let pyxel_sounds = pyxel::sounds();
        let render_channels: Vec<_> =
            self.seqs
                .iter()
                .filter(|seq| !seq.is_empty())
                .map(|seq| {
                    let sounds =
                        seq.iter()
                            .map(|&index| {
                                pyxel_sounds.get(index as usize).cloned().ok_or_else(|| {
                                    "Music contains an invalid sound index".to_string()
                                })
                            })
                            .collect::<Result<_, _>>()?;

                    let channel = Channel::new();
                    audio_mut!(channel).play(sounds, None, true, false)?;
                    Ok(channel)
                })
                .collect::<Result<_, String>>()?;
        drop(pyxel_sounds);

        Audio::save_channels(
            filename,
            &render_channels,
            num_samples,
            use_ffmpeg.unwrap_or(false),
        )
    }
}
