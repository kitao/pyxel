# How to Use Pyxel for Web

Pyxel apps can run on a web browser. No installation of Python or Pyxel is required, and they are accessible from PCs, smartphones, and tablets.

This page explains how to publish Pyxel apps on the web and introduces the web tools available in the browser.

## Publishing Apps on the Web

There are three ways to publish Pyxel apps on the web.

| Method | Features |
| --- | --- |
| [Web Launcher](#web-launcher) | Run directly by specifying a GitHub repository URL. The easiest method |
| [app2html](#app2html) | Convert a Pyxel application (.pyxapp) to HTML for publishing |
| [Custom Tags](#custom-tags) | Embed into existing HTML pages using Pyxel-specific tags |

### Web Launcher

[Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/) is a tool that can directly run Python scripts and Pyxel apps (.pyxapp) published on GitHub by specifying their URL.

Simply enter the required information to automatically generate a launch URL, or manually create one using the format below. Note that Web Launcher always uses the latest version of Pyxel.

#### URL Format

```
https://kitao.github.io/pyxel/wasm/launcher/?<command>=<username>.<repository>.<path>.<filename without extension>
```

Dots (`.`) are used as path separators (e.g., `src/scenes` becomes `src.scenes`).

#### Commands

| Command | Action |
| --- | --- |
| `run` | Run a Python script (.py) |
| `play` | Run a Pyxel app (.pyxapp) |
| `edit` | Open a resource file (.pyxres) in Pyxel Editor |

#### URL Examples

To run `src/scenes/title.py` in user `taro`'s repository `my_repo`:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title
```

To run `dist/games/shooter.pyxapp` in the same repository:

```
https://kitao.github.io/pyxel/wasm/launcher/?play=taro.my_repo.dist.games.shooter
```

#### Attributes

The following attributes can be added to the `run` and `play` commands.

| Attribute | Description |
| --- | --- |
| `gamepad=enabled` | Enable the virtual gamepad (an on-screen controller displayed on touch devices) |
| `packages=pkg1,pkg2` | Specify additional [Pyodide-compatible packages](https://pyodide.org/en/stable/usage/packages-in-pyodide.html) (libraries available in web-based Python) |

Example URL with attributes added to the `run` command:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title&gamepad=enabled&packages=numpy,pandas
```

For the `edit` command, you can specify the startup screen using the `editor` attribute (`image`, `tilemap`, `sound`, `music`).

Example URL for the `edit` command:

```
https://kitao.github.io/pyxel/wasm/launcher/?edit=taro.my_repo.assets.shooter&editor=tilemap
```

Running a multi-file app with `run` may take longer to load. It is recommended to convert it to a `.pyxapp` using the `pyxel package` command and run it with `play`.

### app2html

The `pyxel app2html` command converts a Pyxel application (.pyxapp) into an HTML file.

```sh
pyxel app2html your_app.pyxapp
```

The app data is embedded in the HTML, so you can publish it simply by distributing the generated file.

The Pyxel runtime is pinned to the version used at conversion time, so there is no risk of behavior changes from future updates.

The virtual gamepad is enabled by default (displayed only on touch devices). To disable it, remove `gamepad: "enabled"` from the generated HTML.

### Custom Tags

By writing Pyxel custom tags in an HTML file, you can embed Pyxel apps into existing web pages.

#### Setup

Add the following script tag to your HTML.

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script>
```

To avoid compatibility issues from future updates, you can pin the version by specifying a version number after `@`.

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@v2.7.2/wasm/pyxel.js"></script>
```

#### pyxel-run

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

This method requires [hosting on a server](#running-locally).

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

If `root` is the current directory (`root="."`), the `root` attribute can be omitted.

#### pyxel-play

Runs a Pyxel app (.pyxapp). A URL can also be specified for `root`.

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples/apps"
  name="megaball.pyxapp"
></pyxel-play>
```

#### pyxel-edit

Launches Pyxel Editor. You can specify the startup screen using the `editor` attribute.

```html
<pyxel-edit root="assets" name="shooter.pyxres" editor="image"></pyxel-edit>
```

#### Common Attributes (pyxel-run / pyxel-play only)

| Attribute | Description |
| --- | --- |
| `gamepad="enabled"` | Enable the virtual gamepad (displayed only on touch devices) |
| `packages="pkg1,pkg2"` | Specify additional [Pyodide-compatible packages](https://pyodide.org/en/stable/usage/packages-in-pyodide.html) |

#### Screen Customization

By default, the Pyxel screen is displayed across the entire page. If you place a `<div>` tag with `id="pyxel-screen"` in the HTML, the screen will be rendered inside that element, allowing you to freely adjust its position and size.

#### Running Locally

When using custom tags that load external files, hosting on a server is required. If you have a Python environment, you can use a simple server.

```sh
python -m http.server
# Use python3 on macOS and Linux
```

After starting the server, access `http://localhost:8000/test.html` in your browser.

## Web Tools

Pyxel also provides online tools to help with app development. For detailed usage, refer to the manual on each tool's page.

| Tool | Description |
| --- | --- |
| [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) | An online IDE for creating and running Pyxel apps |
| [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) | A gallery for browsing and running sample code and apps |
| [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) | An editor for composing and playing chiptunes with MML (Music Macro Language) |
| [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/) | A searchable reference for the Pyxel API |
