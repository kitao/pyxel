# How to Use Pyxel for Web

*This document was auto-generated from the [How to Use Pyxel for Web](https://kitao.github.io/pyxel/web/web-usage/) web page, which also offers multilingual support.*

Pyxel for Web is a version of Pyxel that runs in web browsers using WebAssembly technology. No installation of Python or Pyxel is required, and it is accessible from PCs, smartphones, and tablets.

## Ways to Use Pyxel for Web

There are four ways to use Pyxel for Web.

| Method | Description |
| --- | --- |
| Pyxel Code Maker | An online development environment for writing and running code in the browser |
| Pyxel Web Launcher | Run a GitHub repository in the browser by specifying its URL |
| app2html Command | Convert a Pyxel application file (.pyxapp) to an HTML file |
| HTML Custom Tags | Add custom HTML tags to run Pyxel apps on any HTML page |

## Pyxel Code Maker

[Pyxel Code Maker](https://kitao.github.io/pyxel/web/code-maker/) is an online development environment that lets you develop and run Pyxel apps in the browser without installing Python or Pyxel. Just open it in your browser with no setup required, making it a great way to try Pyxel.

For detailed usage, see the [Pyxel Code Maker Manual](https://kitao.github.io/pyxel/web/code-maker/manual.html).

## Pyxel Web Launcher

[Pyxel Web Launcher](https://kitao.github.io/pyxel/web/launcher/) is a tool that runs Python scripts and Pyxel apps (.pyxapp) published on GitHub simply by specifying their URL.

Launch URLs can be created on the [Pyxel Web Launcher](https://kitao.github.io/pyxel/web/launcher/) page by entering the required information, or manually using the format below. Note that Pyxel Web Launcher always uses the latest version of Pyxel.

### URL Format

```text
https://kitao.github.io/pyxel/web/launcher/?<command>=<username>/<repository>/<branch>/<path>/<filename without extension>
```

### Commands

| Command | Action |
| --- | --- |
| `run` | Run a Python script (.py) |
| `play` | Run a Pyxel app (.pyxapp) |
| `edit` | Edit a resource file (.pyxres) in Pyxel Editor |

### URL Examples

To run `src/scenes/title.py` in user `taro`'s repository `my_repo`, branch `main`:

```text
https://kitao.github.io/pyxel/web/launcher/?run=taro/my_repo/main/src/scenes/title
```

To run `dist/games/shooter.pyxapp` in the same repository:

```text
https://kitao.github.io/pyxel/web/launcher/?play=taro/my_repo/main/dist/games/shooter
```

### Additional Parameters

The following additional parameters can be specified for the `run` and `play` commands.

| Parameter | Description |
| --- | --- |
| `gamepad=enabled` | Display a virtual gamepad on touch devices (smartphones and tablets) |

Example URL with additional parameters for the `run` command:

```text
https://kitao.github.io/pyxel/web/launcher/?run=taro/my_repo/main/src/scenes/title&gamepad=enabled
```

The `edit` command has the `editor` parameter to specify which editor to launch: `image`, `tilemap`, `sound`, `music`

Example URL for the `edit` command:

```text
https://kitao.github.io/pyxel/web/launcher/?edit=taro/my_repo/main/assets/shooter&editor=tilemap
```

For apps consisting of multiple Python files, bundling them into a `.pyxapp` file with `pyxel package` and running with `play` will load faster.

## app2html Command

The `pyxel app2html` command converts a Pyxel application file (.pyxapp) into a standalone HTML file.

```sh
pyxel app2html your_app.pyxapp
```

All code and resource data are embedded in the HTML, so the generated file can be published simply by distributing it.

The Pyxel version is pinned to the one used at conversion time, so future updates will not affect behavior.

The virtual gamepad is enabled by default and automatically displayed on touch devices. To disable it, remove `gamepad: "enabled"` from the generated HTML.

## HTML Custom Tags

By adding custom tags (HTML custom elements) provided by Pyxel to an HTML file, you can run Pyxel apps on any HTML page.

### Setup

Add the following script tag to your HTML. If no version is specified, the latest version of Pyxel is always used.

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script>
```

You can pin the Pyxel version by specifying a version number after `@`.

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@v2.8.7/wasm/pyxel.js"></script>
```

Loading this script makes three custom tags available, corresponding to Pyxel's `run`, `play`, and `edit` commands: `pyxel-run`, `pyxel-play`, and `pyxel-edit`.

### pyxel-run

A tag for running Python scripts.

To run Python code directly, write the code in the `script` attribute.

```html
<pyxel-run
  script="
import pyxel
pyxel.init(200, 150)
pyxel.cls(8)
pyxel.line(20, 20, 180, 130, 7)
pyxel.show()
"
></pyxel-run>
```

To load an external file, specify `root` and `name`. `root` is the base directory for file lookup, and `name` is the file path.

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

If `root` is the current directory (`root="."`), the `root` attribute can be omitted.

### pyxel-play

A tag for running Pyxel applications (.pyxapp).

A URL can also be specified for `root`.

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples/apps"
  name="megaball.pyxapp"
></pyxel-play>
```

### pyxel-edit

A tag for editing resource files (.pyxres) in Pyxel Editor.

Use the `editor` attribute to specify which editor to launch.

```html
<pyxel-edit root="assets" name="shooter.pyxres" editor="image"></pyxel-edit>
```

### Common Attributes

The following common attributes are available for `pyxel-run` and `pyxel-play`.

| Attribute | Description |
| --- | --- |
| `gamepad="enabled"` | Enable the virtual gamepad (automatically displayed on touch devices) |

### Customizing Screen Display

By default, the Pyxel screen is displayed across the entire page. Place a `<div>` tag with `id="pyxel-screen"` in the HTML to render the screen inside that element, allowing you to adjust its position and size.

### Running in a Local Environment

When using custom tags that load external files, a server is required because browser security restrictions prevent opening local files directly.

If you have a Python environment, you can start a simple server with the following command.

```sh
python -m http.server
# Use python3 on macOS and Linux
```

After starting the server, access `http://localhost:8000/test.html` in your browser.
