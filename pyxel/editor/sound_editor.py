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
        speed_var
        octave_var
        note_var
        is_playing_var
    """

    def __init__(self, parent):
        super().__init__(parent)
        self._history_data = None
        self.copy_var("help_message_var", parent)

        # Initialize field cursor
        self.field_cursor = FieldCursor(
            max_field_length=MAX_SOUND_LENGTH,
            field_wrap_length=MAX_SOUND_LENGTH,
            get_field=self.get_field,
            add_pre_history=self.add_pre_history,
            add_post_history=self.add_post_history,
        )

        # Initialize octave_var
        self.new_var("octave_var", 2)

        # Initialize is_playing_var
        self.new_var("is_playing_var", None)
        self.add_var_event_listener(
            "is_playing_var", "get", self.__on_is_playing_var_get
        )

        # Initialize sound picker
        self._sound_picker = NumberPicker(
            self, 45, 17, min_value=0, max_value=pyxel.NUM_SOUNDS - 1, value=0
        )
        self._sound_picker.add_event_listener("change", self.__on_sound_picker_change)
        self.add_number_picker_help(self._sound_picker)
        self.copy_var("sound_no_var", self._sound_picker, "value_var")

        # Initialize speed picker
        self._speed_picker = NumberPicker(
            self, 105, 17, min_value=1, max_value=99, value=pyxel.sound(0).speed
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

        # Initialize left octave bar
        self._left_octave_bar = OctaveBar(self, 12, 25)

        # Initialize right octave bar
        self._right_octave_bar = OctaveBar(self, 224, 25)

        # Set event listeners
        self.add_event_listener("undo", self.__on_undo)
        self.add_event_listener("redo", self.__on_redo)
        self.add_event_listener("hide", self.__on_hide)
        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

    @property
    def keyboard_note(self):
        return self._piano_keyboard.note

    def get_field(self, index):
        sound = pyxel.sound(self.sound_no_var)
        if index == 0:
            return sound.notes
        elif index == 1:
            return sound.tones
        elif index == 2:
            return sound.volumes
        elif index == 3:
            return sound.effects
        else:
            return None

    def add_pre_history(self, x, y):
        self._history_data = data = {}
        data["sound_no"] = self.sound_no_var
        data["old_cursor_pos"] = (x, y)
        data["old_field"] = self.field_cursor.field.to_list()

    def add_post_history(self, x, y):
        data = self._history_data
        data["new_cursor_pos"] = (x, y)
        data["new_field"] = self.field_cursor.field.to_list()
        if data["old_field"] != data["new_field"]:
            self.add_history(self._history_data)

    def _play(self):
        self._sound_picker.is_enabled_var = False
        self._speed_picker.is_enabled_var = False
        self._play_button.is_enabled_var = False
        self._stop_button.is_enabled_var = True
        self._loop_button.is_enabled_var = False
        pyxel.play(0, self.sound_no_var, loop=self.should_loop_var)

    def _stop(self):
        self._sound_picker.is_enabled_var = True
        self._speed_picker.is_enabled_var = True
        self._play_button.is_enabled_var = True
        self._stop_button.is_enabled_var = False
        self._loop_button.is_enabled_var = True
        pyxel.stop(0)

    def __on_is_playing_var_get(self, value):
        return pyxel.play_pos(0) is not None

    def __on_sound_picker_change(self, value):
        sound = pyxel.sound(value)
        self._speed_picker.value = sound.speed

    def __on_speed_picker_change(self, value):
        sound = pyxel.sound(self.sound_no_var)
        sound.speed = value

    def __on_play_button_press(self):
        self._play()

    def __on_play_button_mouse_hover(self, x, y):
        self.help_message_var = "PLAY:SPACE"

    def __on_stop_button_press(self):
        self._stop()

    def __on_stop_button_mouse_hover(self, x, y):
        self.help_message_var = "STOP:SPACE"

    def __on_loop_button_mouse_hover(self, x, y):
        self.help_message_var = "LOOP:L"

    def __on_undo(self, data):
        self._stop()
        self.sound_no_var = data["sound_no"]
        self.field_cursor.move_to(*data["old_cursor_pos"])
        self.field_cursor.field.from_list(data["old_field"])

    def __on_redo(self, data):
        self._stop()
        self.sound_no_var = data["sound_no"]
        self.field_cursor.move_to(*data["new_cursor_pos"])
        self.field_cursor.field.from_list(data["new_field"])

    def __on_hide(self):
        self._stop()

    def __on_update(self):
        sound = pyxel.sound(self.sound_no_var)
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
        self.field_cursor.process_input()

    def __on_draw(self):
        self.draw_panel(11, 16, 218, 157)
        pyxel.text(23, 18, "SOUND", TEXT_LABEL_COLOR)
        pyxel.text(83, 18, "SPEED", TEXT_LABEL_COLOR)
