global_instance!(Audio, audio);

pub struct Audio {
    //
}

pub fn init_audio() {
    let audio = Audio {};

    set_instance(audio);
}

impl Audio {
    //
}

/*
class Audio {
public:
Audio();
~Audio();

Sound* GetSoundBank(int32_t sound_index, bool system = false) const;
Music* GetMusicBank(int32_t music_index) const;
int32_t GetPlayPos(int32_t channel) const;
void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
void PlaySound(int32_t channel,
const SoundIndexList& sound_index_list,
bool loop = false);
void PlayMusic(int32_t music_index, bool loop = false);
void StopPlaying(int32_t channel = -1);

private:
SDL_AudioDeviceID audio_device_id_;
Sound** sound_bank_;
Music** music_bank_;
Channel channel_[MUSIC_CHANNEL_COUNT];

static void callback(void* userdata, uint8_t* stream, int len);
}
*/
