import pyxel

from .editor_base import EditorBase
from .field_cursor import FieldCursor
from .octave_bar import OctaveBar
from .piano_keyboard import PianoKeyboard
from .piano_roll import PianoRoll
from .settings import EDITOR_IMAGE, MAX_SOUND_LENGTH, TEXT_LABEL_COLOR
from .sound_field import SoundField
from .widgets import ImageButton, ImageToggleButton, NumberPicker


class SoundEditor(EditorBase):
    """
    Variables:
        sound_no_var
        octave_var
        is_playing_var
        play_pos_var
    """

    def __init__(self, parent):
        super().__init__(parent)
        self._history_data = None
        self.copy_var("help_message_var", parent)

        # octave_var
        self.new_var("octave_var", 2)

        # is_playing_var
        self.new_var("is_playing_var", None)
        self.add_var_event_listener(
            "is_playing_var", "get", self.__on_is_playing_var_get
        )

        # play_pos_var
        self.new_var("play_pos_var", None)
        self.add_var_event_listener("play_pos_var", "get", self.__on_play_pos_get)

        # sound picker
        self._sound_picker = NumberPicker(
            self, 45, 17, min_value=0, max_value=pyxel.NUM_SOUNDS - 1, value=0
        )
        self._sound_picker.add_event_listener("change", self.__on_sound_picker_change)
        self.add_number_picker_help(self._sound_picker)
        self.copy_var("sound_no_var", self._sound_picker, "value_var")

        # speed picker
        self._speed_picker = NumberPicker(
            self, 105, 17, min_value=1, max_value=99, value=pyxel.sound(0).speed
        )
        self._speed_picker.add_event_listener("change", self.__on_speed_picker_change)
        self.add_number_picker_help(self._speed_picker)

        # field cursor
        self.field_cursor = FieldCursor(
            self.get_seq,
            4,
            MAX_SOUND_LENGTH,
            MAX_SOUND_LENGTH,
            self.add_pre_history,
            self.add_post_history,
        )

        # play button
        self._play_button = ImageButton(self, 185, 17, img=EDITOR_IMAGE, u=126, v=0)
        self._play_button.add_event_listener("press", self.__on_play_button_press)
        self._play_button.add_event_listener(
            "mouse_hover", self.__on_play_button_mouse_hover
        )

        # stop button
        self._stop_button = ImageButton(
            self, 195, 17, img=EDITOR_IMAGE, u=135, v=0, is_enabled=False
        )
        self._stop_button.add_event_listener("press", self.__on_stop_button_press)
        self._stop_button.add_event_listener(
            "mouse_hover", self.__on_stop_button_mouse_hover
        )

        # loop button
        self._loop_button = ImageToggleButton(
            self, 205, 17, img=EDITOR_IMAGE, u=144, v=0, is_checked=False
        )
        self._loop_button.add_event_listener(
            "mouse_hover", self.__on_loop_button_mouse_hover
        )

        # piano keyboard
        self._piano_keyboard = PianoKeyboard(self)

        # piano roll
        self._piano_roll = PianoRoll(self)

        # sound field
        self._sound_field = SoundField(self)

        # left octave bar
        self._left_octave_bar = OctaveBar(self, 12, 25)

        # right octave bar
        self._right_octave_bar = OctaveBar(self, 224, 25)

        # event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("hide", self.__on_hide)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    @property
    def keyboard_note(self):
        return self._piano_keyboard.note

    def get_seq(self, index):
        sound = pyxel.sound(self.sound_no_var)
        if index == 0:
            data = sound.notes
        elif index == 1:
            data = sound.tones
        elif index == 2:
            data = sound.volumes
        elif index == 3:
            data = sound.effects
        return data

    def add_pre_history(self, x, y):
        self._history_data = data = {}
        data["sound_no"] = self.sound_no_var
        data["cursor_before"] = (x, y)
        data["before"] = self.field_cursor.seq[:]

    def add_post_history(self, x, y):
        data = self._history_data
        data["cursor_after"] = (x, y)
        data["after"] = self.field_cursor.seq[:]
        if data["before"] != data["after"]:
            self.add_history(self._history_data)

    def _play(self):
        self._sound_picker.is_enabled_var = False
        self._speed_picker.is_enabled_var = False
        self._play_button.is_enabled_var = False
        self._stop_button.is_enabled_var = True
        self._loop_button.is_enabled_var = False
        pyxel.play(0, self.sound_no_var, loop=self.is_looping_var)

    def _stop(self):
        self._sound_picker.is_enabled_var = True
        self._speed_picker.is_enabled_var = True
        self._play_button.is_enabled_var = True
        self._stop_button.is_enabled_var = False
        self._loop_button.is_enabled_var = True
        pyxel.stop(0)

    def __on_is_playing_var_get(self, value):
        return pyxel.play_pos(0) is not None

    def __on_play_pos_get(self, value):
        play_pos = pyxel.play_pos(0)
        if play_pos is None:
            return -1
        else:
            return play_pos[0]

    def __on_sound_picker_change(self, value):
        sound = pyxel.sound(value)
        self._speed_picker.value = sound.speed

    def __on_speed_picker_change(self, value):
        sound = pyxel.sound(self.sound_no_var)
        sound.speed = value

    def __on_play_button_press(self):
        self._play()

    def __on_play_button_mouse_hover(self, x, y):
        self.parent.help_message_var = "PLAY:SPACE"

    def __on_stop_button_press(self):
        self._stop()

    def __on_stop_button_mouse_hover(self, x, y):
        self.parent.help_message_var = "STOP:SPACE"

    def __on_loop_button_mouse_hover(self, x, y):
        self.parent.help_message_var = "LOOP:L"

    def __on_undo(self, data):
        self._stop()
        self._sound_picker.value_var = data["sound"]
        self.field_cursor.move(*data["cursor_before"])
        self.field_cursor.data[:] = data["before"]

    def __on_redo(self, data):
        self._stop()
        self._sound_picker.value_var = data["sound"]
        self.field_cursor.move(*data["cursor_after"])
        self.field_cursor.data[:] = data["after"]

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        sound = pyxel.sound(self._sound_picker.value_var)
        if self._speed_picker.value_var != sound.speed:
            self._speed_picker.value_var = sound.speed

        if pyxel.btnp(pyxel.KEY_SPACE):
            if self.is_playing_var:
                self._stop_button.is_pressed_var = True
                return
            else:
                self._play_button.is_pressed_var = True

        if not self._play_button.is_enabled_var:
            self._stop()
        if self._loop_button.is_enabled_var and pyxel.btnp(pyxel.KEY_L):
            self._loop_button.is_pressed_var = True
        if pyxel.btnp(pyxel.KEY_PAGEUP):
            self.octave = min(self.octave + 1, 3)
        if pyxel.btnp(pyxel.KEY_PAGEDOWN):
            self.octave = max(self.octave - 1, 0)

        self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", TEXT_LABEL_COLOR)
        pyxel.text(83, 18, "SPEED", TEXT_LABEL_COLOR)
