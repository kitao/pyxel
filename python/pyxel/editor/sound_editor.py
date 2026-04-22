import pyxel

from .editor_base import EditorBase
from .field_cursor import FieldCursor
from .octave_bar import OctaveBar
from .piano_keyboard import PianoKeyboard
from .piano_roll import PianoRoll
from .settings import EDITOR_IMAGE, MAX_SOUND_LENGTH, TEXT_LABEL_COLOR
from .sound_field import SoundField
from .widgets import ImageButton, ImageToggleButton, NumberPicker

_SOUND_FIELDS = ("notes", "tones", "volumes", "effects")

_FIELD_HELP = (
    "NOTE:CLICK/PIANO_KEY+ENTER/BS/DEL",
    "TONE:T/S/P/N/BS/DEL",
    "VOLUME:0-7/BS/DEL",
    "EFFECT:N/S/V/F/H/Q/BS/DEL",
)


class SoundEditor(EditorBase):
    # Variables:
    #   sound_index_var
    #   speed_var
    #   octave_var
    #   note_var
    #   is_playing_var

    def __init__(self, parent):
        super().__init__(parent)
        self._history_data = None
        self.copy_var("help_message_var", parent)

        # Initialize field cursor
        self.field_cursor = FieldCursor(
            self,
            max_field_length=MAX_SOUND_LENGTH,
            field_wrap_length=MAX_SOUND_LENGTH,
            max_field_values=[59, 3, 7, 5],
            get_field=self.get_field,
            add_pre_history=self.add_pre_history,
            add_post_history=self.add_post_history,
            enable_cross_field_copy=False,
        )

        self.new_var("octave_var", 2)

        self.new_var("is_playing_var", None)
        self.add_var_event_listener(
            "is_playing_var", "get", self.__on_is_playing_var_get
        )

        # Initialize sound picker
        self._sound_picker = NumberPicker(
            self, 45, 17, min_value=0, max_value=pyxel.NUM_SOUNDS - 1, value=0
        )
        self._sound_picker.add_event_listener("change", self.__on_sound_picker_change)
        self._sound_picker.add_event_listener(
            "mouse_hover", self.__on_sound_picker_mouse_hover
        )
        self.add_number_picker_help(self._sound_picker)
        self.copy_var("sound_index_var", self._sound_picker, "value_var")

        # Initialize speed picker
        self._speed_picker = NumberPicker(
            self, 105, 17, min_value=1, max_value=99, value=pyxel.sounds[0].speed
        )
        self._speed_picker.add_event_listener("change", self.__on_speed_picker_change)
        self.add_number_picker_help(self._speed_picker)
        self.copy_var("speed_var", self._speed_picker, "value_var")

        # Initialize play button
        self._play_button = ImageButton(self, 185, 17, img=EDITOR_IMAGE, u=126, v=0)
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

        # Initialize piano keyboard
        self._piano_keyboard = PianoKeyboard(self)
        self.copy_var("note_var", self._piano_keyboard)

        # Initialize piano roll
        self._piano_roll = PianoRoll(self)

        # Initialize sound field
        self._sound_field = SoundField(self)

        # Initialize octave bars
        self._left_octave_bar = OctaveBar(self, 12, 25)
        self._right_octave_bar = OctaveBar(self, 224, 25)

        # Set event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("hide", self.__on_hide)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    # Public methods

    def get_field(self, index):
        if 0 <= index < len(_SOUND_FIELDS):
            return getattr(pyxel.sounds[self.sound_index_var], _SOUND_FIELDS[index])
        return None

    def add_pre_history(self, x=None, y=None, *, bank_copy=False):
        data = {}
        self._history_data = data
        data["sound_index"] = self.sound_index_var
        if bank_copy:
            data["old_speed"] = self.speed_var
            data["old_data"] = [list(self.get_field(i)) for i in range(4)]
        else:
            data["old_cursor_pos"] = (x, y)
            data["old_field"] = list(self.field_cursor.field)

    def add_post_history(self, x=None, y=None, *, bank_copy=False):
        data = self._history_data
        if bank_copy:
            data["new_speed"] = self.speed_var
            data["new_data"] = [list(self.get_field(i)) for i in range(4)]
            if (
                data["new_speed"] != data["old_speed"]
                or data["new_data"] != data["old_data"]
            ):
                self.add_history(data)
        else:
            data["new_cursor_pos"] = (x, y)
            data["new_field"] = list(self.field_cursor.field)
            if data["new_field"] != data["old_field"]:
                self.add_history(data)

    def get_field_help_message(self):
        if self.field_cursor.is_selecting:
            return "COPY:CTRL+A/C/X/V SHIFT:CTRL+U/D"
        cursor_y = self.field_cursor.y
        return _FIELD_HELP[cursor_y] if cursor_y < len(_FIELD_HELP) else ""

    # Helpers

    def _play(self, is_partial):
        self._sound_picker.is_enabled_var = False
        self._speed_picker.is_enabled_var = False
        self._play_button.is_enabled_var = False
        self._stop_button.is_enabled_var = True
        self._loop_button.is_enabled_var = False

        tick = self.field_cursor.x * self.speed_var if is_partial else 0
        pyxel.play(
            0,
            self.sound_index_var,
            sec=tick / 120,
            loop=self.should_loop_var,
        )

    def _stop(self):
        self._sound_picker.is_enabled_var = True
        self._speed_picker.is_enabled_var = True
        self._play_button.is_enabled_var = True
        self._stop_button.is_enabled_var = False
        self._loop_button.is_enabled_var = True
        pyxel.stop(0)

    def _restore_state(self, data, prefix):
        self._stop()
        self.sound_index_var = data["sound_index"]
        if f"{prefix}_data" in data:
            pyxel.sounds[self.sound_index_var].speed = data[f"{prefix}_speed"]
            for i in range(4):
                self.get_field(i)[:] = data[f"{prefix}_data"][i]
        else:
            self.field_cursor.move_to(*data[f"{prefix}_cursor_pos"], False)
            self.field_cursor.field[:] = data[f"{prefix}_field"]

    # Event handlers

    def __on_is_playing_var_get(self, value):
        return pyxel.play_pos(0) is not None

    def __on_sound_picker_change(self, value):
        self._speed_picker.value = pyxel.sounds[value].speed

    def __on_sound_picker_mouse_hover(self, _x, _y):
        self.help_message_var = "COPY_ALL:CTRL+SHIFT+C/X/V"

    def __on_speed_picker_change(self, value):
        pyxel.sounds[self.sound_index_var].speed = value

    def __on_play_button_press(self):
        self._play(pyxel.btn(pyxel.KEY_SHIFT))

    def __on_play_button_mouse_hover(self, _x, _y):
        self.help_message_var = "PLAY:SPACE PART-PLAY:SHIFT+SPACE"

    def __on_stop_button_press(self):
        self._stop()

    def __on_stop_button_mouse_hover(self, _x, _y):
        self.help_message_var = "STOP:SPACE"

    def __on_loop_button_mouse_hover(self, _x, _y):
        self.help_message_var = "LOOP:L"

    def __on_undo(self, data):
        self._restore_state(data, "old")

    def __on_redo(self, data):
        self._restore_state(data, "new")

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        sound = pyxel.sounds[self.sound_index_var]
        if self.speed_var != sound.speed:
            self.speed_var = sound.speed

        if pyxel.btnp(pyxel.KEY_SPACE):
            if self.is_playing_var:
                self._stop_button.is_pressed_var = True
                return
            else:
                self._play_button.is_pressed_var = True

        if not self._play_button.is_enabled_var and not self.is_playing_var:
            self._stop()

        if self._loop_button.is_enabled_var and pyxel.btnp(pyxel.KEY_L):
            self.should_loop_var = not self.should_loop_var

        if pyxel.btnp(pyxel.KEY_PAGEUP):
            self.octave_var = min(self.octave_var + 1, 3)
        if pyxel.btnp(pyxel.KEY_PAGEDOWN):
            self.octave_var = max(self.octave_var - 1, 0)

        if not self.is_playing_var:
            self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", TEXT_LABEL_COLOR)
        pyxel.text(83, 18, "SPEED", TEXT_LABEL_COLOR)
