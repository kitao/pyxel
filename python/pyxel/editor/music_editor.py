import pyxel

from .editor_base import EditorBase
from .field_cursor import FieldCursor
from .music_field import MusicField
from .settings import EDITOR_IMAGE, MAX_MUSIC_LENGTH, TEXT_LABEL_COLOR
from .sound_selector import SoundSelector
from .widgets import ImageButton, ImageToggleButton, NumberPicker


class MusicEditor(EditorBase):
    """
    Variables:
        music_index_var
        should_loop_var
        is_playing_var
        help_message_var
    """

    def __init__(self, parent):
        super().__init__(parent)
        self.copy_var("help_message_var", parent)

        # Initialize is_playing_var
        self.new_var("is_playing_var", False)

        # Initialize field cursor
        self.field_cursor = FieldCursor(
            self,
            max_field_length=MAX_MUSIC_LENGTH,
            field_wrap_length=16,
            max_field_values=[pyxel.NUM_SOUNDS - 1] * 4,
            get_field=self.get_field,
            add_pre_history=self.add_pre_history,
            add_post_history=self.add_post_history,
            enable_cross_field_copy=True,
        )

        # Initialize music picker
        self._music_picker = NumberPicker(
            self, 45, 17, min_value=0, max_value=pyxel.NUM_MUSICS - 1, value=0
        )
        self._music_picker.add_event_listener(
            "mouse_hover", self.__on_music_picker_mouse_hover
        )
        self.add_number_picker_help(self._music_picker)
        self.copy_var("music_index_var", self._music_picker, "value_var")

        # Initialize play button
        self._play_button = ImageButton(
            self,
            185,
            17,
            img=EDITOR_IMAGE,
            u=126,
            v=0,
        )
        self._play_button.add_event_listener("press", self.__on_play_button_press)
        self._play_button.add_event_listener(
            "mouse_hover", self.__on_play_button_mouse_hover
        )

        # Initialize stop button
        self._stop_button = ImageButton(
            self, 195, 17, img=EDITOR_IMAGE, u=135, v=0, is_enabled=False
        )
        self._stop_button.add_event_listener("press", self.__on_stop_button_press)
        self._stop_button.add_event_listener(
            "mouse_hover", self.__on_stop_button_mouse_hover
        )

        # Initialize loop button
        self._loop_button = ImageToggleButton(
            self, 205, 17, img=EDITOR_IMAGE, u=144, v=0, is_checked=False
        )
        self._loop_button.add_event_listener(
            "mouse_hover", self.__on_loop_button_mouse_hover
        )
        self.copy_var("should_loop_var", self._loop_button, "is_checked_var")

        # Initialize music field
        self._music_field = [MusicField(self, 11, 29 + i * 25, i) for i in range(4)]

        # Initialize sound selector
        self._sound_selector = SoundSelector(self)

        # Set event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("hide", self.__on_hide)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    def get_field(self, index):
        if index >= pyxel.NUM_CHANNELS:
            return

        music = pyxel.musics[self.music_index_var]
        seqs_len = len(music.seqs)

        if seqs_len < pyxel.NUM_CHANNELS:
            seqs = music.seqs.to_list()
            seqs.extend([[] for _ in range(pyxel.NUM_CHANNELS - seqs_len)])
            music.seqs.from_list(seqs)
        elif seqs_len > pyxel.NUM_CHANNELS:
            seqs = music.seqs.to_list()
            del seqs[pyxel.NUM_CHANNELS :]
            music.seqs.from_list(seqs)

        return music.seqs[index]

    def add_pre_history(self, x=None, y=None, *, bank_copy=False):
        self._history_data = data = {}
        data["music_index"] = self.music_index_var

        if bank_copy:
            data["old_data"] = [self.get_field(i).to_list() for i in range(4)]
        else:
            data["old_cursor_pos"] = (x, y)
            data["old_field"] = self.field_cursor.field.to_list()

    def add_post_history(self, x=None, y=None, *, bank_copy=False):
        data = self._history_data

        if bank_copy:
            data["new_data"] = [self.get_field(i).to_list() for i in range(4)]
            if data["new_data"] != data["old_data"]:
                self.add_history(data)
        else:
            data["new_cursor_pos"] = (x, y)
            data["new_field"] = self.field_cursor.field.to_list()
            if data["new_field"] != data["old_field"]:
                self.add_history(data)

    def _play(self, is_partial):
        self.is_playing_var = True
        self._music_picker.is_enabled_var = False
        self._play_button.is_enabled_var = False
        self._stop_button.is_enabled_var = True
        self._loop_button.is_enabled_var = False

        tick = 0
        if is_partial:
            for i in range(self.field_cursor.x):
                music = pyxel.musics[self.music_index_var]
                sound = pyxel.sounds[music.seqs[self.field_cursor.y][i]]
                tick += len(sound.notes) * sound.speed

        pyxel.playm(self.music_index_var, tick=tick, loop=self.should_loop_var)

    def _stop(self):
        self.is_playing_var = False
        self._music_picker.is_enabled_var = True
        self._play_button.is_enabled_var = True
        self._stop_button.is_enabled_var = False
        self._loop_button.is_enabled_var = True

        pyxel.stop()

    def __on_music_picker_mouse_hover(self, x, y):
        self.help_message_var = "COPY_ALL:CTRL+SHIFT+C/X/V"

    def __on_play_button_press(self):
        self._play(pyxel.btn(pyxel.KEY_SHIFT))

    def __on_stop_button_press(self):
        self._stop()

    def __on_play_button_mouse_hover(self, x, y):
        self.help_message_var = "PLAY:SPACE PART-PLAY:SHIFT+SPACE"

    def __on_stop_button_mouse_hover(self, x, y):
        self.help_message_var = "STOP:SPACE"

    def __on_loop_button_mouse_hover(self, x, y):
        self.help_message_var = "LOOP:L"

    def __on_undo(self, data):
        self._stop()
        self.music_index_var = data["music_index"]

        if "old_data" in data:
            for i in range(4):
                self.get_field(i).from_list(data["old_data"][i])
        else:
            self.field_cursor.move_to(*data["old_cursor_pos"], False)
            self.field_cursor.field.from_list(data["old_field"])

    def __on_redo(self, data):
        self._stop()
        self.music_index_var = data["music_index"]

        if "new_data" in data:
            for i in range(4):
                self.get_field(i).from_list(data["new_data"][i])
        else:
            self.field_cursor.move_to(*data["new_cursor_pos"], False)
            self.field_cursor.field.from_list(data["new_field"])

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        if self.is_playing_var:
            self.is_playing_var = None
            for i in range(pyxel.NUM_CHANNELS):
                if pyxel.play_pos(i) is not None:
                    self.is_playing_var = True
                    break

        if pyxel.btnp(pyxel.KEY_SPACE):
            if self.is_playing_var:
                self._stop_button.is_pressed_var = True
            else:
                self._play_button.is_pressed_var = True

        if self.is_playing_var:
            return

        if not self._play_button.is_enabled_var:
            self._stop()

        if self._loop_button.is_enabled_var and pyxel.btnp(pyxel.KEY_L):
            self.should_loop_var = not self.should_loop_var

        if not self.is_playing_var:
            self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 9)
        pyxel.text(23, 18, "MUSIC", TEXT_LABEL_COLOR)
