# How to Use the Web Version of Pyxel

With the Web version of Pyxel, you can run Pyxel applications on a web browser from a PC, smartphone, or tablet without needing to install Python or Pyxel.

There are three ways to use the Web version of Pyxel.

- **Specify a GitHub repository in Pyxel Web Launcher**<br>
  By specifying the name of a GitHub repository in the URL of the Pyxel Web Launcher, the repository is directly loaded, and you can run the app in a web browser. This is the easiest way if the app is published on GitHub.

- **Convert a Pyxel app to an HTML file**<br>
  If your app is in Pyxel application format (.pyxapp), you can convert it to an HTML file using `pyxel app2html` command. The resulting HTML file can be run standalone without needing a server.

- **Create an HTML file using Pyxel custom tags**<br>
  You can create an HTML file to run an app using Pyxel-specific custom tags. The HTML file needs to be hosted on a server, but it allows for embedding into existing HTML pages and various customizations.

Each method is explained below.

## Specify a GitHub Repository in Pyxel Web Launcher

If your Python code or Pyxel app (.pyxapp) is published on GitHub, you can run it directly using Pyxel Web Launcher.

The URL format for Pyxel Web Launcher is as follows:

```
https://kitao.github.io/pyxel/wasm/launcher/?<Command>=<GitHub username>.<Repository Name>.<App Directories>.<File Name Without Extension>
```

There are three available commands.

- `run`: Execute a Python script
- `play`: Run a Pyxel app
- `edit`: Launch Pyxel Editor

For example, if the username is `taro`, the repository is named `my_repo`, the file directory is `src/scenes`, and the Python script is `title.py`, the URL would be:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title
```

If you want to run a `shooter.pyxapp` located in the `dist/games` directory, the URL would be:

```
https://kitao.github.io/pyxel/wasm/launcher/?play=taro.my_repo.dist.games.shooter
```

If an app is split into multiple files, running it with the `run` command may take longer to load. In that case, it is recommended to convert it into a Pyxel application file (.pyxapp) and run it with the `play` command.

The `run` and `play` commands can have additional attributes such as `gamepad` to enable a virtual gamepad and `packages` to specify additional packages.

For example, if you want to enable the virtual gamepad and use NumPy and Pandas as additional packages, the URL would be:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title&gamepad=enabled&packages=numpy,pandas
```

Note that the packages that can be added are limited to those supported by [Packages built in Pyodide](https://pyodide.org/en/stable/usage/packages-in-pyodide.html).

When using the `edit` command, you can specify the Pyxel Editor's startup screen using the `editor` attribute.

For example, to open the `shooter.pyxres` file located in the `assets` directory with the Tilemap Editor screen, use the following URL:

```html
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.assets.shooter&editor=tilemap
```

On the [Pyxel Web Launcher page](https://kitao.github.io/pyxel/wasm/launcher/), you can enter the required information to automatically generate a launch URL for your app.

You can also create a URL to play MML by entering multi-channel MML in the MML List, separated by semicolons (`;`) like `CDE;EFG`.

## Convert a Pyxel App to an HTML File

You can convert a Pyxel application file (.pyxapp) into a standalone HTML file using the following command:

```sh
pyxel app2html your_app.pyxapp
```

The generated HTML file has the virtual gamepad enabled by default, but you can disable it by editing the custom tags in the HTML file.

## Create an HTML File Using Pyxel Custom Tags

You can run a Pyxel app by writing Pyxel-specific custom tags in an HTML file.

To use Pyxel custom tags, add the following script tag to your HTML file:

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script>
```

To run Python code directly, specify the code in the script attribute of the `pyxel-run` tag, as shown below:

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

To run external Python files, specify the `root` and `name` attributes in the `pyxel-run` tag.

`root` is the directory where the search begins, and `name` is the file path.

For example, if you save the above code as `test.py` in the same directory as the HTML file, write the following:

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

If `root` is the current directory (`root="."`), the `root` attribute can be omitted.

To read external files from a local HTML file, you need to host it on a server.

If you have a Python environment, you can start a simple server with the following command:

```python
python -m http.server
# use python3 instead of python for mac and linux
```

After starting the server, you can access it in a browser at `http://localhost:8000/test.html`.

Similarly, you can run a Pyxel app (.pyxapp) using the `pyxel-play` tag:

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples"
  name="megaball.pyxapp"
></pyxel-play>
```

In this example, the `root` attribute specifies a URL.

Both the `pyxel-run` and `pyxel-play` tags support the `gamepad` attribute to enable a virtual gamepad and the `packages` attribute to specify additional packages.

For example, to enable the virtual gamepad and use NumPy and Pandas, write the following:

```html
<pyxel-run name="test.py" gamepad="enabled" packages="numpy,pandas"></pyxel-run>
```

The available packages are limited to those supported by [Packages built in Pyodide](https://pyodide.org/en/stable/usage/packages-in-pyodide.html).

You can also launch Pyxel Editor using the `pyxel-edit` tag.

For example, to edit the `shooter.pyxres` file in the `assets` directory with the Image Editor screen, write the following:

```html
<pyxel-edit root="assets" name="sample.pyxres" editor="image"></pyxel-edit>
```

If you add a `<div>` tag with `id="pyxel-screen"` to an HTML file running Pyxel, that element will be used as the Pyxel screen. By adjusting the position and size of this `<div>` tag, you can change the placement and dimensions of the Pyxel screen.

## Pinning the Pyxel Version

By default, the web version of Pyxel always loads the latest version from the server, so future updates may cause your existing code to stop working.

To prevent this, specify the Pyxel version in your HTML file to pin the version you want to use.

For example, in your HTML code:

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@2.4/wasm/pyxel.js"></script>
```

or

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@2.4.6/wasm/pyxel.js"></script>
```

Specify the version number you want to pin in the URL as shown above.
