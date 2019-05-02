#include "pyxelcore/resource.h"

namespace pyxelcore {

void Resource::LoadAsset(const char* filename) {
  /*
    @staticmethod
    def load(filename):
        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        with gzip.open(filename, mode="rb") as fp:
            pickled_data = fp.read()

        data = pickle.loads(pickled_data)

        # todo: version check

        image_list = data.get("image")
        if image_list:
            for i in range(RENDERER_IMAGE_COUNT - 1):
                pyxel.image(i).data[:, :] = pickle.loads(image_list[i])

        tilemap_list = data.get("tilemap")
        if tilemap_list:
            if type(tilemap_list[0]) is tuple:
                for i in range(RENDERER_TILEMAP_COUNT):
                    tilemap = pyxel.tilemap(i)
                    tilemap.data[:, :] = pickle.loads(tilemap_list[i][0])
                    tilemap.refimg = tilemap_list[i][1]
            else:  # todo: delete this block in the future
                for i in range(RENDERER_TILEMAP_COUNT):
                    pyxel.tilemap(i).data[:, :] = pickle.loads(tilemap_list[i])

        sound_list = data.get("sound")
        if sound_list:
            for i in range(AUDIO_SOUND_COUNT - 1):
                src = sound_list[i]
                dest = pyxel.sound(i)

                dest.note[:] = src.note
                dest.tone[:] = src.tone
                dest.volume[:] = src.volume
                dest.effect[:] = src.effect
                dest.speed = src.speed

        music_list = data.get("music")
        if music_list:
            for i in range(AUDIO_MUSIC_COUNT - 1):
                src = music_list[i]
                dest = pyxel.music(i)

                dest.ch0[:] = src.ch0
                dest.ch1[:] = src.ch1
                dest.ch2[:] = src.ch2
                dest.ch3[:] = src.ch3
  */
}

void Resource::SaveAsset(const char* filename) {
  /*
    @staticmethod def save(filename):
        data = {"version": pyxel.VERSION}

        image_list = [
            pyxel.image(i).data.dumps() for i in range(RENDERER_IMAGE_COUNT - 1)
        ]
        data["image"] = image_list

        tilemap_list = [
            (pyxel.tilemap(i).data.dumps(), pyxel.tilemap(i).refimg)
            for i in range(RENDERER_TILEMAP_COUNT)
        ]
        data["tilemap"] = tilemap_list

        sound_list = [pyxel.sound(i) for i in range(AUDIO_SOUND_COUNT - 1)]
        data["sound"] = sound_list

        music_list = [pyxel.music(i) for i in range(AUDIO_MUSIC_COUNT - 1)]
        data["music"] = music_list

        pickled_data = pickle.dumps(data)

        dirname = os.path.dirname(inspect.stack()[-1].filename)
        filename = os.path.join(dirname, filename)

        with gzip.open(filename, mode="wb") as fp:
            fp.write(pickled_data)
  */
}

}  // namespace pyxelcore
