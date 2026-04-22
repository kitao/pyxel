import sys
from pathlib import Path

import pyxel

from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .settings import APP_HEIGHT, APP_WIDTH, EDITOR_IMAGE, HELP_MESSAGE_COLOR
from .sound_editor import SoundEditor
from .tilemap_editor import TilemapEditor
from .widgets import ImageButton, RadioButton, Widget
from .widgets.settings import (
    WIDGET_BACKGROUND_COLOR,
    WIDGET_HOLD_TIME,
    WIDGET_PANEL_COLOR,
    WIDGET_REPEAT_TIME,
    WIDGET_SHADOW_COLOR,
)

_EDITOR_TYPES = {"image": 0, "tilemap": 1, "sound": 2, "music": 3}


class App(Widget):
    # Variables:
    #   editor_type_var
    #   help_message_var

    def __init__(self, resource_file, starting_editor):
        # Resolve the absolute path before pyxel.init changes the working directory
        original_resource_file = resource_file
        resource_path = Path(resource_file).absolute()

        if resource_path.is_dir():
            print(f"A directory named '{original_resource_file}' exists")
            sys.exit(1)
        if not resource_path.parent.is_dir():
            print(f"Directory for '{original_resource_file}' does not exist")
            sys.exit(1)

        pyxel.init(APP_WIDTH, APP_HEIGHT, quit_key=pyxel.KEY_NONE)
        pyxel.mouse(True)
        colors = list(pyxel.colors)
        self._set_title(original_resource_file)

        resource_file = str(resource_path)
        if resource_path.exists():
            pyxel.load(resource_file)
        else:
            pyxel.load_pal(resource_file)

        # Concatenate system and user palettes so colors[:NUM_COLORS] are the
        # system set and colors[NUM_COLORS:] are the user set
        pyxel.num_user_colors = len(pyxel.colors)
        colors += list(pyxel.colors)
        pyxel.colors[:] = colors

        super().__init__(None, 0, 0, pyxel.width, pyxel.height)
        self._resource_file = resource_file

        self.new_var("help_message_var", "")

        # Initialize editor button
        self._editor_button = RadioButton(
            self,
            1,
            1,
            img=EDITOR_IMAGE,
            u=0,
            v=0,
            num_buttons=4,
            value=_EDITOR_TYPES.get(starting_editor, 0),
        )
        self._editor_button.add_event_listener("change", self.__on_editor_button_change)
        self._editor_button.add_event_listener(
            "mouse_hover", self.__on_editor_button_mouse_hover
        )
        self.copy_var("editor_type_var", self._editor_button, "value_var")

        # Initialize undo button
        self._undo_button = ImageButton(self, 48, 1, img=EDITOR_IMAGE, u=36, v=0)
        self._undo_button.add_event_listener("press", self.__on_undo_button_press)
        self._undo_button.add_event_listener(
            "mouse_hover", self.__on_undo_button_mouse_hover
        )

        # Initialize redo button
        self._redo_button = ImageButton(self, 57, 1, img=EDITOR_IMAGE, u=45, v=0)
        self._redo_button.add_event_listener("press", self.__on_redo_button_press)
        self._redo_button.add_event_listener(
            "mouse_hover", self.__on_redo_button_mouse_hover
        )

        # Initialize save button
        self._save_button = ImageButton(self, 75, 1, img=EDITOR_IMAGE, u=54, v=0)
        self._save_button.add_event_listener("press", self.__on_save_button_press)
        self._save_button.add_event_listener(
            "mouse_hover", self.__on_save_button_mouse_hover
        )

        # Initialize editors
        self._editors = [
            ImageEditor(self),
            TilemapEditor(self),
            SoundEditor(self),
            MusicEditor(self),
        ]
        self.__on_editor_button_change(self.editor_type_var)

        self.add_event_listener("update", self.__on_update)
        self.add_event_listener("draw", self.__on_draw)

        pyxel.run(self.update_all, self.draw_all)

    # Helpers

    @property
    def _editor(self):
        return self._editors[self.editor_type_var]

    @staticmethod
    def _set_title(resource_file):
        pyxel.title(f"Pyxel Editor - {resource_file}")

    # Event handlers

    def __on_editor_button_change(self, value):
        for i, editor in enumerate(self._editors):
            editor.is_visible_var = i == value

    def __on_editor_button_mouse_hover(self, _x, _y):
        self.help_message_var = "EDITOR:ALT+LEFT/RIGHT"

    def __on_undo_button_press(self):
        self._editor.undo()

    def __on_undo_button_mouse_hover(self, _x, _y):
        self.help_message_var = "UNDO:CTRL+Z"

    def __on_redo_button_press(self):
        self._editor.redo()

    def __on_redo_button_mouse_hover(self, _x, _y):
        self.help_message_var = "REDO:CTRL+Y"

    def __on_save_button_press(self):
        pyxel.save(self._resource_file)

    def __on_save_button_mouse_hover(self, _x, _y):
        self.help_message_var = "SAVE:CTRL+S"

    def __on_update(self):
        # pyxel._dropped_files is the legacy path for drop events injected by
        # the WASM runtime; drain it each frame and prefer native dropped_files
        wasm_dropped_files = getattr(pyxel, "_dropped_files", [])
        pyxel._dropped_files = []
        if pyxel.dropped_files:
            dropped_file = pyxel.dropped_files[-1]
        elif wasm_dropped_files:
            dropped_file = wasm_dropped_files[-1]
        else:
            dropped_file = None
        if dropped_file:
            file_ext = Path(dropped_file).suffix
            if file_ext == pyxel.RESOURCE_FILE_EXTENSION:
                pyxel.stop()
                for editor in self._editors:
                    editor.reset_history()
                pyxel.load(dropped_file)
                self._set_title(dropped_file)
            else:
                self._editor.trigger_event("drop", dropped_file)

        if pyxel.btn(pyxel.KEY_ALT):
            if pyxel.btnp(pyxel.KEY_LEFT):
                self.editor_type_var = (self.editor_type_var - 1) % len(self._editors)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.editor_type_var = (self.editor_type_var + 1) % len(self._editors)

        self._undo_button.is_enabled_var = self._editor.can_undo
        self._redo_button.is_enabled_var = self._editor.can_redo

        if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
            if pyxel.btnp(pyxel.KEY_S):
                self._save_button.is_pressed_var = True
            if self._editor.can_undo and pyxel.btnp(
                pyxel.KEY_Z, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self._undo_button.is_pressed_var = True
            elif self._editor.can_redo and pyxel.btnp(
                pyxel.KEY_Y, hold=WIDGET_HOLD_TIME, repeat=WIDGET_REPEAT_TIME
            ):
                self._redo_button.is_pressed_var = True

        # Hidden save shortcut for Pyxel Code Maker
        if pyxel.btn(pyxel.KEY_F13):
            self._save_button.is_pressed_var = True

    def __on_draw(self):
        pyxel.cls(WIDGET_BACKGROUND_COLOR)
        pyxel.rect(0, 0, 240, 9, WIDGET_PANEL_COLOR)
        pyxel.line(0, 9, 239, 9, WIDGET_SHADOW_COLOR)
        pyxel.text(93, 2, self.help_message_var, HELP_MESSAGE_COLOR)
        self.help_message_var = ""
