import os
import pathlib

import pyxel
import pyxel.cli

APP_LAUNCHER_ENV = "PYXEL_APP_LAUNCHER"
APPS_DIR = (pathlib.Path(__file__).parent / "apps").resolve()
APP_NAMES = [
    "30sec_of_daylight",
    "megaball",
    "vortexion",
    "laser-jetman",
    "space_rescue",
    "mega_wing",
    "cursed_caverns",
    "8bit-bgm-gen",
]

ROW_COUNT = 5
ROW_HEIGHT = 12


class App:
    def __init__(self):
        pyxel.init(418, 173, title="Pyxel App Launcher")
        pyxel.integer_scale(True)

        self.umplus10 = pyxel.Font("assets/umplus_j10r.bdf")
        self.cursor_index = self.cursor_pos = 0
        self.view_pos = -0.5

        self.list_view = pyxel.Image(400, ROW_HEIGHT * ROW_COUNT)

        self.apps = []
        for name in APP_NAMES:
            path = APPS_DIR / f"{name}.pyxapp"
            metadata = pyxel.cli.get_pyxel_app_metadata(str(path))
            metadata["name"] = name
            metadata["filepath"] = str(path)
            self.apps.append(metadata)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_UP, 15, 6) or pyxel.btnp(
            pyxel.GAMEPAD1_BUTTON_DPAD_UP, 15, 6
        ):
            self.cursor_index = max(self.cursor_index - 1, 0)

        if pyxel.btnp(pyxel.KEY_DOWN, 15, 6) or pyxel.btnp(
            pyxel.GAMEPAD1_BUTTON_DPAD_DOWN, 15, 6
        ):
            self.cursor_index = min(self.cursor_index + 1, len(self.apps) - 1)

        if any(
            pyxel.btnp(key)
            for key in [
                pyxel.KEY_RETURN,
                pyxel.GAMEPAD1_BUTTON_A,
                pyxel.GAMEPAD1_BUTTON_START,
            ]
        ):
            os.environ[APP_LAUNCHER_ENV] = self.apps[self.cursor_index]["filepath"]
            # Prevent the launched app from inheriting the window state
            os.environ.pop(pyxel.WINDOW_STATE_ENV, None)
            pyxel.reset()

        self.cursor_pos = self.cursor_pos * 0.7 + self.cursor_index * 0.3
        self.view_pos = min(
            max(self.view_pos, self.cursor_pos - ROW_COUNT + 1.5),
            self.cursor_pos - 0.5,
        )

    def draw(self):
        pyxel.cls(0)

        # Draw instruction
        pyxel.text(
            61,
            8,
            "UP/DOWN: SELECT APP    ENTER: LAUNCH APP    ALT(OPT)+R: RETURN TO LAUNCHER",
            3,
        )

        # Draw list
        self.list_view.cls(0)
        self.list_view.camera(0, ROW_HEIGHT * self.view_pos)
        for i in range(ROW_COUNT + 1):
            index = pyxel.floor(self.view_pos) + i
            if 0 <= index < len(self.apps):
                self.list_view.text(
                    4,
                    ROW_HEIGHT * index,
                    self.apps[index]["name"],
                    9,
                    self.umplus10,
                )

        pyxel.blt(
            9,
            17,
            self.list_view,
            0,
            0,
            self.list_view.width,
            self.list_view.height,
        )
        pyxel.rectb(8, 16, 402, 62, 3)

        # Draw cursor
        focus_y = ROW_HEIGHT * (self.cursor_pos - self.view_pos)
        pyxel.pal(0, 10)
        pyxel.pal(9, 0)
        pyxel.blt(
            9,
            17 + focus_y,
            self.list_view,
            0,
            focus_y,
            self.list_view.width,
            ROW_HEIGHT,
        )
        pyxel.pal()

        # Draw metadata
        metadata = self.apps[self.cursor_index]
        for i, key in enumerate(
            [key for key in metadata.keys() if key not in ["name", "filepath"]]
        ):
            pyxel.text(
                13,
                90 + ROW_HEIGHT * i,
                f"{key.capitalize():7}: {metadata[key]}",
                13,
                self.umplus10,
            )
        pyxel.rectb(8, 86, 402, 79, 3)


def switch_app():
    app = os.environ.pop(APP_LAUNCHER_ENV, None)
    if app:
        pyxel.cli.play_pyxel_app(app)
    else:
        # Prevent the launcher app from inheriting the window state
        os.environ.pop(pyxel.WINDOW_STATE_ENV, None)
        App()


switch_app()
