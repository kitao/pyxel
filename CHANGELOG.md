# Change Log

## 2.5.5

- Changed exe packaging with PyInstaller for the reset function
- Fixed touch device detection for Firefox in the web version
- Changed the mechanism of the reset function and the play command
- Changed to remove extra directories after the app2exe command
- Improved automatic file download for the web version
- Adjusted the error output display size in the web version
- Updated the reset function mechanism

## 2.5.4

- Added two Pyxel apps by Adam for the app launcher
- Fixed Example 17 Python command execution issue
- Fixed the reset function issue when called inside pyxapp
- Updated the design of the web pages
- Updated the zip crate to version 5.0

## 2.5.3

- Changed the HTML pages for Pyxel to use the latest Pyxel from CDN
- Added a gamepad shortcut for the reset operation
- Excluded GIF and ZIP files from Pyxel application files
- Added START and BACK buttons to the virtual gamepad for the web

## 2.5.2

- Fixed cargo publish error by adding features sdl2_bundle
- Added an environment variable for the reset function's window state
- Added three sample games from the Pyxel book
- Added Example 17 for the app launcher and the reset function
- Updated the pyo3 crate to version 0.26
- Added gamepad support to Example 15

## 2.5.1

- Changed to preserve environment variables on reset
- Fixed cargo publish error by adding features sdl2
- Added line break support for custom font rendering
- Fixed app2exe issue with white spaces

## 2.5.0

- Refactored the platform abstraction layer
- Reduced error output in the web version
- Added the reset function
- Added automatic use of old_mml when '~' is used
- Fixed delayed sound playback on Android browsers

## 2.4.10

- Fixed parameter commands ignored after repeat in MML

## 2.4.9

- Fixed dot note length bug in MML parser
- Added support for tie notation with numbers only in MML

## 2.4.8

- Fixed playback when all sounds in the array are empty
- Added console output to the mml command in Pyxel Web Launcher

## 2.4.7

- Fixed a vibrato bug when the sound speed is low

## 2.4.6

- Added the mml command to Pyxel Web Launcher
- Changed the note interpolation time to 1ms
- Pinned the Pyxel version used by the app2html command
- Updated the web usage instructions
- Updated the sysinfo crate to version 0.36

## 2.4.5

- Added call to old_mml method when the old syntax is detected

## 2.4.4

- Fixed a cargo login warning
- Restored the tick option of the play and playm functions
- Added documentation on pinning the Pyxel version for the web version
- Cleaned up and improved usability of Example 14
- Updated the toml crate to version 0.9
- Updated Pyodide to version 0.27.7

## 2.4.3

- Added note interpolation processing to suppress click noise
- Restored the excl options in the load and save functions

## 2.4.2

- Reverted the add_delta in blip_buf to prevent audio degradation

## 2.4.1

- Renamed the noise field of the Tone class to mode
- Added the sample_bits field to the Tone class
- Made the wavetable field of the Tone class support arbitrary length
- Renamed the tone_index parameter of the Tone command in MML to tone
- Changed the types of the Sound class members
- Switched to the blip_buf crate
- Added asterisk parameter support to the @GLI command in MML
- Removed redundant MML code from Example 9

## 2.4.0

- Fixed the audio module initialize arguments
- Updated Pyodide to version 0.27.5
- Added a Q&A about saving application data to the FAQ
- Updated the zip crate to version 4.0
- Updated the serde-xml-rs to version 0.8
- Updated the pyo3 crate to version 0.25
- Updated the sysinfo crate to version 0.35
- Updated the bindgen crate to version 0.72
- Fixed GitHub Actions to use Rust nightly-2025-02-01
- Renewed the sound engine and MML syntax
- Renamed the waveform field of the Tone class to wavetable
- Changed the play and playm functions to use sec instead of tick
- Changed the play_pos function to return sec instead of note_no
- Changed Sound and Music save methods to use sec instead of count
- Added the total_sec method to the Sound class
- Changed the mml method of the Sound class to use the new MML syntax
- Added the old_mml method to the Sound class for the old MML syntax
- Added MML string support to the play function
- Added MML string support to the play method of the Channel class
- Removed the colors, tones, and channels from the resource format
- Renamed the excl options to exclude in the load and save functions
- Removed the incl options from the load and save functions
- Updated Example 9 to use new MML syntax
- Changed the default floating-point type from f64 to f32

## 2.3.18

- Added SDL2 include paths for Linux
- Fixed relative path handling in the watch command
- Updated the message image
- Added a screen size specification to the README files
- Added blank lines to format code
- Updated the sysinfo crate to version 0.34
- Changed math functions to static
- Added the DEFAULT_COLORS constant
- Updated the Pyxel thanks image

## 2.3.17

- Changed btn-related functions to use assert
- Removed autoplay code from the web version

## 2.3.16

- Modified the audio resume processing for the web version

## 2.3.15

- Organized the FAQ section
- Fixed audio resuming in the web version

## 2.3.14

- Added warnings for invalid keys in btn-related functions
- Added version number output on startup in the web version
- Updated the image crate to version 0.25

## 2.3.13

- Modified the inclusion order of SDL.h
- Fixed a bug that broke the app2html command

## 2.3.12

- Added a Q&A about file loading to the FAQ
- Added support for overriding screen position and size in the web version
- Updated the pyo3 crate to version 2.4

## 2.3.11

- Removed the Google Analytics links from the web pages
- Updated the file download check in the web version

## 2.3.10

- Fixed stack overflow issue in the fill function
- Handled XMLHttpRequest exceptions in the web version

## 2.3.9

- Added support for loading upper-level files in the web version
- Adjusted click message removal timing in the web version
- Updated Pyodide to version 0.27.3
- Stopped downloading unnecessary files in the web version
- Fixed a warning in a utility script

## 2.3.8

- Fixed local module imports in the web version
- Updated the usage instructions for the web version

## 2.3.7

- Added a script for Pyxel User Examples pages
- Added support for local module imports in the web version
- Updated Rust to version nightly-2025-02-01

## 2.3.6

- Changed rotation in blt and bltm to clockwise

## 2.3.5

- Updated the GitHub Action scripts
- Updated the rand crate to version 0.9
- Updated the rand_xoshiro to version 0.7
- Updated 8bit BGM Generator to version 1.30
- Fixed multi-gamepad support
- Fixed the text function ignoring camera when font set
- Aligned Emscripten version with Pyodide
- Fixed the input_text variable
- Stopped using the once_cell crate

## 2.3.4

- Downgraded Pyodide to version 0.27.0

## 2.3.3

- Removed NoSleep.js from the web version of Pyxel
- Fixed a bug in the save method of the Music class
- Changed the location of the image used for MP4 creation
- Restored the links to the Discord servers in the README files

## 2.3.2

- Added the hound crate
- Added the save method to the Sound and Music classes

## 2.3.1

- Fixed a bug in loading old resource files
- Added the `X` command to MML
- Renamed the `!` command in MML to `~`
- Added support for adding multiple dots to a note in MML
- Restored the input_keys variable
- Updated Example 9 to use MML for music setup
- Updated 8bit BGM Generator to version 1.22

## 2.3.0

- Added the ToneIndex type
- Adjusted the size of sound-related types
- Added the mml method to the Sound class
- Updated the directories crate to version 6.0
- Changed the sample rate to 22.05kHz
- Reduced click noise
- Updated Pyodide to version 0.27.1
- Updated the year in the LICENSE files

## 2.2.11

- Fixed types in the pyi file
- Fixed an input issue in the sound editor
- Formatted sound strings in examples

## 2.2.10

- Updated the pyo3 crate to version 2.3
- Reduced the sound clock rate from 120MHz to 2.048MHz

## 2.2.9

- Added the LICENSE file to the Python package
- Excluded the pycache directory from the copy_examples command
- Updated the message image for the README files
- Fixed clippy warnings

## 2.2.8

- Modified a shortcut description in the README files
- Renamed (tile_x, tile_y) to (image_tx, image_ty) in the README files
- Replaced the usage of a deprecated API
- Changed the tilemap editor to load Layer 0 when a TMX file is dropped onto it
- Updated Maturin to the latest version
- Updated Pyodide to version 0.26.4
- Updated the indexmap crate to version 2.7
- Updated the once_cell crate to version 1.20
- Updated the zip crate to version 2.2
- Updated the sysinfo crate to version 0.33
- Updated the glow crate to version 0.16
- Updated the bindgen crate to version 0.71
- Raised the minimum supported macOS to version 13
- Fixed a bug in the mouse cursor position

## 2.2.7

- Updated Pyodide to version 0.26.3
- Added the perf_monitor function
- Added the integer_scale function
- Renamed the argument of the fullscreen function
- Added the integer-scale toggle feature with Alt(Option)+8
- Added the gamepad shortcuts using A+B+X+Y+DL/DR/DU/DD
- Changed the default scaling from integer to maximum

## 2.2.6

- Renamed WORKING_DIR to BASE_DIR
- Added the user_data_dir function
- Switched from the platform-dir crate to the directories crate
- Updated the glow crate to version 0.15
- Fully revised the translations of all README files

## 2.2.5

- Fixed the displayed color issue caused by the sRGB setting
- Added a note regarding the usage of the run command on the web

## 2.2.4

- Fixed a bug when playing a pyxapp with the same process ID
- Updated the sysinfo crate to version 0.25
- Updated the license description in the README files
- Updated the instructions for using the web version of Pyxel
- Updated the Q&A

## 2.2.3

- Updated the description of Pyxel's features in the README files
- Ensured that the metadata is in UTF-8 format
- Added the pyxel.cli.get_pyxel_app_metadata function
- Added the pyxel.cli.print_pyxel_app_metadata function
- Fixed a warning on macOS Sonoma
- Fixed new clippy warnings

## 2.2.2

- Fixed the mypy errors
- Changed an image layout in the README files
- Updated Python in GitHub Actions to version 3.12
- Added support for adding metadata to a Pyxel application file
- Added metadata to the bundled Pyxel application files

## 2.2.1

- Added the watch command description to the README files
- Removed an unnecessary line in Example 14
- Added the Font class
- Added a font option to the text function
- Changed Example 14 to use the native font rendering

## 2.2.0

- Removed the keyword-only arguments
- Added the rotate and scale options to the blt and bltm functions
- Specified Maturin to version 1.7.0 to prevent linking errors
- Modified the function notation in Example 4
- Added Example 16 for rotation and scaling

## 2.1.10

- Fixed a color rendering issue on Windows
- Replaced links to Twitter with X in the README files
- Updated the bindgen crate to version 0.70

## 2.1.9

- Updated the required Python to version 3.8 or higher
- Avoided using the gil-refs feature in the pyo3 crate
- Fixed key state changes during special inputs

## 2.1.8

- Updated Emscripten to version 3.1.61
- Updated SDL2 to version 2.28.4
- Updated the pyo3 crate to version 0.22
- Fixed keyword-only arguments functionality

## 2.1.7

- Modified help messages in Pyxel Editor
- Changed the return value type of the sgn function to integer
- Fixed the push back process in Example 10 and 15
- Prevented editing during playback in Pyxel Editor
- Fixed incorrect array references during playback in Pyxel Editor
- Updated the sysinfo crate to version 0.31

## 2.1.6

- Updated the message image for the README files
- Added Turkish and Ukrainian READMEs
- Fixed a warning on macOS Sonoma
- Updated Pyodide to version 0.26.2
- Updated the glow crate to version 0.14

## 2.1.5

- Updated the description of the set_effects method in the README files
- Added a value change shortcut to the sound and music editors
- Changed the initial value for the noise sound register
- Suppressed the outdated resource file version warning
- Changed the mutex control for sound playback

## 2.1.4

- Added a bank copy feature to Pyxel Editor
- Fixed the version check for the resource file

## 2.1.3

- Incremented the resource format version

## 2.1.2

- Updated the descriptions of the pget and pset functions
- Added the Half-FadeOut and Quarter-FadeOut effects to the Sound class
- Fixed the warp_mouse function

## 2.1.1

- Fixed the resume option of the play function
- Removed the non-functional CTRL+Drop feature from Pyxel Editor
- Updated the zip crate version

## 2.1.0

- Fixed a help message in Pyxel Editor
- Added a resume option to the play function
- Changed the function notation in Example 4
- Changed Example 9 to use the resume option for SFX playback
- Added the descriptions of the resume option to the README files
- Updated Pyodide to version 0.26.1

## 2.0.14

- Fixed the bltm referencing out of range

## 2.0.13

- Updated the make update command
- Fixed the app2exe and app2html commands

## 2.0.12

- Fixed installation instructions for Mac in the README files
- Modified build instructions in Makefile
- Updated Pyodide to version 0.25.1
- Updated Emscripten to version 3.1.53

## 2.0.11

- Added error messages for the pyxel command
- Updated crate versions

## 2.0.10

- Changed the location of pyproject.toml and requirements.txt
- Fixed the sqrt function
- Fixed a non pixel perfect bug for OpenGL ES
- Added support for encodings other than UTF-8 in the app2exe command

## 2.0.9

- Fixed the timing to disable the slide effect
- Fixed the release script

## 2.0.8

- Changed the directory structure of the project
- Organized project metadata for Rust and Python
- Disabled the slide effect on the first note of a sound
- Fixed clippy warnings

## 2.0.7

- Turned off the high DPI mode for performance perspective
- Added a shortcut to output the current color palette
- Added load_tmx and load method to the Tilemap class
- Enabled importing a TMX file via drag and drop in the tilemap editor
- Changed destination for image drag-and-drop in the image editor
- Refined the code for Example 9 and Example 10
- Added an incl_colors option to the from_image method of the Image class
- Added an incl_colors option to the load method of the Image class
- Added Example 15
- Refined Example 10
- Fixed a color count change bug on OpenGL ES

## 2.0.6

- Added support for high DPI mode
- Changed how to determine whether to use OpenGL or OpenGL ES

## 2.0.5

- Restored the publish of the crate to the release script
- Fixed a bug in Pyxel Editor when creating new resource files

## 2.0.4

- Fixed a mouse wheel bug
- Added a shortcut to output an image bank

## 2.0.3

- Increased the audio clock rate to 120MHz
- Changed the mouse cursor position when focus is lost

## 2.0.2

- Changed the order of declarations in the pyi file
- Music.set no longer requires specifying all channels
- Switched to Ruff for linting and formatting Python code
- Added the usage of the show and flip functions to the README files
- Added Example 14

## 2.0.1

- Removed the publish of the crate to the release script
- Changed the type of tile coordinates from u16 back to u8
- Removed source code path from the binary
- Renamed Waveform and waveforms to Tone and tones
- Updated the resource file format for the tones
- Changed CDN links to use the latest Pyxel explicitly
- Changed Sound.set_tones to accept digits
- Added Example 14 (still under development)

## 2.0.0

- Switched to the C version of SDL2
- Added support for resizing the colors list
- Switched screen rendering to GLSL
- Updated Pyodide to version 0.24.1
- Updated Emscripten to version 3.1.45
- Updated SDL2 to 2.24.2
- Added the screen_mode function to change screen rendering type
- Added a shortcut to change the screen mode with Alt(Option)+9
- Added support for the third and fourth gamepads
- Added the dither function to set dithering type
- Added images, tilemaps as system lists
- Marked the image and tilemap functions as deprecated functions
- Added channels, sounds, and musics as system lists
- Marked the channel, sounds, and music functions as deprecated functions
- Renamed the reset_capture function to reset_screencast
- Renamed the set_mouse_pos function to warp_mouse
- Renamed the drop_files variable to dropped_files
- Removed the is_fullscreen variable
- Removed the input_keys variable
- Removed the set_btn and set_btnv functions
- Integrated the image and refimg of Tilemap into imgsrc
- Marked the image and refimg of Tilemap as deprecated fields
- Renamed snds_list of Music to seqs
- Marked the snds_list of Music as a deprecated field
- Changed to a new resource format based on TOML
- Changed arguments to the load and save functions
- Added Waveform class for waveform editing
- Added waveforms as a system list
- Added a detune field to Channel
- Updated 8bit BGM generator to the latest version

## 1.9.18

- Added the windowed and icon options to PyInstaller

## 1.9.17

- Replaced an image in the README files
- Fixed the app2exe command to include modules correctly
- Added support for Raspberry Pi (ARMv7)
- Updated Emscripten to version 3.1.42
- Updated Pyodide to version 0.23.3
- Updated crate versions

## 1.9.16

- Disabled quit by ESC key in Pyxel Editor
- Fixed the nseed function
- Bundled 8bit BGM generator by frenchbread
- Added BGMs by 8bit BGM generator to Example 9

## 1.9.15

- Changed the platform specification method in Rust for FreeBSD
- Switched to the official Pyodide instead of the customized version
- Updated Emscripten to version 3.1.36
- Expanded the supported macOS 10.12+ (Intel), 11.0+ (Apple Silicon)

## 1.9.14

- Modified the layout of Example 13
- Fixed bounding box handling for BDF font rendering

## 1.9.13

- Added URL generator to Pyxel Web Launcher
- Added a data_ptr method to the Image and Tilemap class
- Updated Emscripten to version 3.1.34
- Added Example 13

## 1.9.12

- Added support for environments without game controllers
- Adjusted the initial gain of the sound
- Updated Emscripten to version 3.1.31

## 1.9.11

- Changed Makefile to lint Emscripten code
- Fixed clippy warnings
- Modified the instructions for the Pyxel.colors list
- Added error messages for the pyxel command
- Fixed the default filename of Pyxel Editor for web

## 1.9.10

- Switched to the SDL2 clear function
- Fixed virtual and real gamepads working at the same time

## 1.9.9

- Stopped using the array macro
- Fixed clippy warnings
- Updated Emscripten to version 3.1.29
- Updated Pyodide to version 0.22.0
- Fixed gamepad input bug
- Added support for dynamic addition of gamepads
- Changed a help message for the tilemap editor

## 1.9.8

- Modified description for Example 13
- Added system palette colors besides normal palette colors
- Fixed to keep the default palette color in Pyxel Editor
- Updated Emscripten to version 3.1.28
- Updated the source of Pyodide-SDL2

## 1.9.7

- Simplified utility scripts
- Added show command description to the README files
- Restored the flip function for non-web only
- Added Example 99 (for non-web only)
- Added Pyxel palette file (.pyxpal) support

## 1.9.6

- Added a link to the code to the examples list
- Changed the save location of the watch command information
- Added instructions on how to install Pyxel on Mac
- Adjusted window resizing timing of the watch command

## 1.9.5

- Fixed to work without a body tag in Pyxel Web
- Added the app2html command to create an HTML page

## 1.9.4

- Fixed error handling of the pyxel command
- Added the app2exe command to create an executable

## 1.9.3

- Limited depth to be checked to 3 for the watch command
- Fixed the package command to accept various file specifications
- Removed extra files from sample Pyxel application files
- Added running message to the watch command

## 1.9.2

- Fixed the watch command to work on Windows

## 1.9.1

- Added the watch command for live coding
- Renamed PYXEL_VERSION to VERSION
- Renamed PYXEL_WORKING_DIR to WORKING_DIR
- Added the WATCH_INFO_FILE constant
- Fixed filename for downloads in Pyxel Web
- Changed the way to specify a file in the package command
- Fixed some statements in the README files

## 1.9.0

- Added a video and a message to users to the README files
- Updated Pyodide
- Updated Emscripten to version 3.1.24

## 1.8.22

- Added file save function for the web
- Fixed to prevent the web browser Ctrl+S action in Pyxel Editor

## 1.8.21

- Switched to CSS variables for the web
- Removed unnecessary sleep for the web

## 1.8.20

- Stopped setting the position style for the screen div
- Added a packages attribute to the script_test example

## 1.8.19

- Fixed URLs in the launcher page
- Fixed resizing of screen elements correctly
- Suppressed screen flickering at startup for the web

## 1.8.18

- Fixed the name of the startup script to be main
- Changed file structure and launcher URL

## 1.8.17

- Limited control restriction for the web
- Separated the color scheme for the web

## 1.8.16

- Added access analytics code for the web pages
- Added support to launch Pyxel Web from JavaScript
- Added support to change a screen div
- Fixed the bltm figure for the README files
- Added display of a message when a screenshot fails

## 1.8.15

- Changed the splash image size for the web
- Suppressed position shift at startup in Mobile Chrome

## 1.8.14

- Added the link to Web examples in the README files
- Added the packages attribute to HTML custom elements
- Updated Pyodide-SDL2 to support additional packages

## 1.8.13

- Added support to specify the Pyxel version in CDN links
- Fixed link to the examples
- Added the starting editor parameter to Pyxel Editor

## 1.8.12

- Added support to auto-load files without the pyxel-asset element

## 1.8.11

- Switched to the latest CDN
- Added support to cancel frame skip when elapsed time is large
- Fixed the version number check

## 1.8.10

- Added Pyxel Web Launcher
- Added support to show Python error messages on the web

## 1.8.9

- Changed the color scheme for the web
- Added virtual gamepad support for the web
- Added gamepad support to examples
- Added the gamepad attribute to HTML custom elements
- Modified key assigns for Pyxel application files

## 1.8.8

- Added support to wait for user input on the PC web
- Changed the server script to detect Windows

## 1.8.7

- Fixed file existence check in Pyxel Editor
- Fixed sound playback on mobile web devices
- Removed the onstart attribute from the custom elements
- Added support to display the Pyxel logo while loading
- Added "tap to start" menu for the web on mobile devices

## 1.8.6

- Changed to not release the sdist wheel
- Refined the show function
- Switched to emscripten_force_exit for quit
- Suppressed an exception error message on the web
- Fixed a flip bug in Pyxel Editor
- Added directory existence check to Pyxel Editor

## 1.8.5

- Fixed the links to the examples in the README files
- Added utility functions to the script for the web
- Added custom elements for the web

## 1.8.4

- Modified the background color
- Made the Pyxel on WASM page compatible with dark mode
- Fixed the canvas height on iOS
- Separated Pyodide-SDL2 into a separate repository
- Changed to use Pyodide-SDL2 via CDN
- Added instructions for the Web platform to the README files

## 1.8.3

- Added a script to build and copy Pyodide
- Changed the audio buffer size for WASM
- Added a utility module for WASM
- Removed the module_search_path option from CLI
- Changed functions for CLI to public
- Fixed music not stopping on quit on WASM
- Removed the flip function for WASM support
- Fixed the mouse coordinates in WASM
- Changed to release the sdist wheel as well
- Changed the background color to be brighter
- Added links for Pyxel on WASM to the README files

## 1.8.2

- Fixed to release crates correctly
- Added 32-bit Linux support
- Added a README-abspath

## 1.8.1

- Added 32-bit Windows support
- Added ARM64 Linux support
- Switched to manylinux to build Linux wheels
- Fixed links in the README files

## 1.8.0

- Changed to include the blip-buf module
- Renamed inner SDL2 module to PlatformSdl2
- Added fall death to Example 10
- Fixed Example 12 to not capture the screen
- Consolidated tool settings into pyproject.toml
- Added support to work without audio
- Changed to build wheels for each platform
- Added experimental web browser support

## 1.7.2

- Fixed function names in the README files
- Added a configuration for isort
- Added flip features to Pyxel Editor
- Fixed clippy warnings

## 1.7.1

- Fixed the description for Tilemap.set in the README files
- Fixed the link to Example 12 in the README files
- Fixed the examples layout in the README files
- Updated links for Discord servers in the README files
- Changed the way to set crate versions
- Added a display_scale option to the init function
- Changed the default window size slightly smaller

## 1.7.0

- Fixed type hint definitions
- Changed the location of working directory
- Added Ctrl+A and Ctrl+X shortcuts for the image/tilemap editor
- Added copy and paste feature for the sound/music editor
- Renamed the sequences property of the Music class to snds_list
- Added Example 12
- Updated external library versions
- Fixed a parameter name of the btnp function

## 1.6.9

- Fixed the select tool of Pyxel Editor
- Refactored the drawing functions
- Added utility functions for math
- Fixed the play command bug on Windows

## 1.6.8

- Fixed mouse input not being received when the window is activated
- Fixed the window to be active when a file is dropped
- Added the elli and ellib functions
- Added the fill function
- Added support to work even when audio is not installed
- Updated the README files

## 1.6.7

- Fixed a file drop bug
- Changed to pause only when minimized

## 1.6.6

- Integrated the release and public actions
- Added the PYXEL_WORKING_DIR constant
- Added a mechanism to check the latest version
- Fixed a bug of the play_pos function
- Added a tick option to the play and playm functions
- Added a partial playback feature to Pyxel Editor
- Fixed mouse click handling in the sound editor
- Updated the README files

## 1.6.5

- Fixed the GitHub Actions workflow to publish

## 1.6.2

- Fixed the play command to remove working directories
- Fixed the key repeat bug
- Added the GitHub Actions workflow to build
- Removed the cli function
- Updated the README files

## 1.6.1

- Fixed the pyxel play command on Windows
- Updated the README files

## 1.6.0

- Fixed the pip command option in the README files
- Ensured key inputs are detected
- Removed the unused click event from the Widget class
- Bundled the arcade ball physics game by Adam

## 1.5.8

- Fixed a pyxapp to be included in Python wheel
- Fixed clippy warnings

## 1.5.7

- Fixed Example 11 images
- Added the module search path option to the pyxel command
- Changed the default install directory on Windows
- Added tests for the package and play options
- Renamed setbtn, setbtnv, and setmpos
- Changed fullscreen to take an argument
- Added the is_fullscreen variable
- Registered Pyxel to GitHub Sponsors
- Bundled the 1st Pyxel Jam winning game by Adam
- Updated the README files

## 1.5.6

- Fixed the categories of the Pyxel crates
- Fixed the key input bug of flip
- Added shortcuts for the tilemap editor
- Added Example 11
- Changed the Makefile to be usable in MinGW shell
- Updated the README files

## 1.5.5

- Fixed to record the screen video with collect interval
- Renamed the input setters to setbtn, setbtnv and setmpos
- Changed the input setters to get floating numbers
- Changed the key definitions to SDL_Keycode base
- Updated the example videos
- Updated Pyxel Editor videos
- Modified the melody of Example 10

## 1.5.4

- Added the file operation error messages
- Refined the Python code
- Updated the README in Korean
- Updated the README in Portuguese
- Changed to stop updating when the window is inactive
- Changed quit to end the application immediately
- Fixed the animated GIF recorder
- Modified the vibrato depth parameter
- Added the SFX and BGM to Example 10
- Fixed the play button bug in the sound editor
- Fixed the editing method for the piano roll
- Added the capture_scale option to the init
- Added the scale option to the screenshot and screencast
- Improved the sound playback response
- Added the screen video of Example 10

## 1.5.3

- Fixed the tilemap editor
- Replaced the asset for Example 10
- Fixed the typo in the README files

## 1.5.2

- Added the camera function
- Changed the arguments of the bltm function
- Added support for crates.io
- Updated the README in Portuguese
- Added the script to update the version number
- Fixed the type hints for the optional arguments
- Fixed the Makefile for ARM Linux
- Added support for GLIBC 2.27

## 1.5.1

- Updated the README in German
- Updated the README in Chinese
- Updated the README in Korean
- Updated the README in Spanish
- Updated the README in French
- Fixed file permission error when running pyxapp
- Fixed an argument name of the blt function
- Fixed a bug when self-copying with the blt function
- Changed the release build method for Makefile
- Restored Pyxel Editor to quit with Esc key
- Fixed a bug of the scrolling area of Pyxel Editor
- Supported the Universal Binary for M1 Mac
- Reduced the git repository size

## 1.5.0

- Re-implemented the core engine in Rust
- Changed to statically link SDL2 libraries for Mac
- Renamed the key constants to the same as SDL2
- Added the pyxel command to work standalone without Python
- Added the cli function to launch command line interface
- Added support for Pyxel application file (.pyxapp)
- Added the installer for Windows
- Simplified the init function arguments
- Removed maximum screen size limit
- Enabled changing the maximum capture time to reduce reserved memory
- Added support for loading various image formats other than PNG
- Optimized GIF animation compression
- Enabled adding image banks and tilemap banks dynamically
- Added drawing methods to the Image and Tilemap classes
- Changed the tile format of tilemap to tuple of (image_tx, image_ty)
- Renamed the properties of the Sound and Music class
- Changed the play_pos function to return a tuple of sound and note
- Added support for dynamic palette change with the colors list
- Added the input_keys and input_text variables to obtain the entered key
- Added the drop_files variable to obtain the dropped files
- Added the icon function to set the application icon
- Added the title function to set the application title
- Added the fullscreen function to toggle fullscreen manually
- Added the Channel class which can control the channel volume
- Added the functions to overwrite key inputs and mouse position
- Added the functions to capture screen manually
- Added Example 10
- Fixed setup.py so that images are referenced correctly on PyPI page
- Added the flip function
- Added the pyi file for type hinting

## 1.4.4

- Added the README in French

## 1.4.3

- Added the README in Italian
- Added the README in Russian
- Fixed a crash bug when playing sound

## 1.4.2

- Updated the installation instructions in the README files
- Changed gcc version for Mac
- Added Example 9
- Added the Noguchi's tilemap for reference
- Added figures for the API reference to the README files

## 1.4.1

- Changed to use gcc compiler on Mac
- Fixed the icon to not get affected by palette changes

## 1.4.0

- Changed the required version of Python
- Changed the quit behavior of the Pyxel application

## 1.3.9

- Updated the compiler version to C++17
- Added support for multi-byte character paths

## 1.3.8

- Modified .gitignore for Windows
- Changed the installation of Py installer to optional
- Changed the way to set the path on Windows
- Updated the pipfile

## 1.3.7

- Updated the library download script
- Updated SDL2 version for Windows
- Improved the animated GIF making method

## 1.3.6

- Added the quit key to Example 8
- Fixed the key input detection at the first frame
- Fixed the way to quit the Pyxel application

## 1.3.5

- Changed the way to quit the Pyxel application

## 1.3.4

- Updated the README in Korean
- Updated the installation instructions in the README files
- Changed color names along the new palette
- Changed to optimize an animated GIF with Gifsicle

## 1.3.3

- Fixed the way to quit the Pyxel application
- Fixed Python version check

## 1.3.2

- Updated the installation instructions in the README files
- Added the link to the Discord server to the README files
- Added variable frame rate support for animated GIF
- Added the mouse_wheel variable
- Added the fullscreen option to the init function
- Changed the way to quit the Pyxel application
- Removed the border options of the init function
- Changed the required version of Python

## 1.3.1

- Improved the animated GIF making method
- Added the README in Portuguese
- Added support for Python 3.7 on Windows
- Changed the color change shortcuts to ignore the alt keys

## 1.3.0

- Fixed the version check of the resource file
- Fixed the typo of the PURPLE variables
- Added the uninitialized error
- Added support for command key shortcuts in Pyxel Editor
- Fixed undo and redo of the sound/music editor
- Changed color comparison method when importing images
- Updated the SDL to 2.0.10
- Updated the SDL_image to 2.0.5
- Changed dll search method for Python 3.8
- Updated the READMEs
- Added KEY_NONE constant to ignore key input
- Added the pget function and renamed pix to pset
- Changed the palette colors
- Added new Pyxel palette file
- Changed the animated GIF making method

## 1.2.10

- Added the tri and trib functions
- Modified the install option in the README files
- Added the quit_key option to the init function
- Added the target options to the load functions
- Added the partial load function to Pyxel Editor
- Added Example 8
- Modified Example 5

## 1.2.9

- Added the Korean version of the README

## 1.2.8

- Fixed the Pyxel Packager

## 1.2.7

- Added the Chinese version of the README
- Added the icon option to the Pyxel Packager command
- Fixed the copy method of the Tilemap class

## 1.2.6

- Updated the installation instructions in the README files
- Removed dependency on NumPy
- Changed the search path of the asset folder in the Pyxel Packager
- Fixed the undo/redo for copy and paste in Pyxel Editor

## 1.2.5

- Fixed the pitch of the sound being off

## 1.2.4

- Fixed to keep the mouse cursor speed on Linux
- Added Python version check

## 1.2.3

- Fixed an error of tone playback in the sound editor
- Fixed the image index of tilemaps being reset in Pyxel Editor

## 1.2.2

- Updated the requirements.txt and Pipfile
- Fixed the Pyxel Packager for Windows

## 1.2.1

- Switched from SDL_Scancode to SDL_Keycode
- Fixed to use the correct separator in the Pyxel Packager

## 1.2.0

- Removed support for loading old format
- Added the build method to the README files
- Added the usage of the show and flip functions to the README files
- Added the Pyxel Packager command

## 1.1.8

- Added Example 7
- Fixed the set method of the Music class
- Added the list of the examples to the README files
- Added the show function

## 1.1.7

- Fixed to stop with ctrl-c
- Updated the classifiers of setup.py
- Added API descriptions to the README files
- Added the constants for the default colors
- Fixed to stop the application with Python exception

## 1.1.6

- Changed the way to make module properties
- Added public constants for fonts and banks
- Removed the screen size limit
- Added a description of a shortcut

## 1.1.5

- Optimized the sound and music functions
- Added the color class for the default palette
- Added the shortcut to select a color to the image editor
- Fixed the sound preview bug of the sound editor
- Enabled quitting from anywhere
- Added the flip function

## 1.1.4

- Fixed the index check of the playm function
- Added support to access the screen as the image bank #4
- Changed the area to display the mouse cursor
- Optimized the image and tilemap functions
- Updated the README files

## 1.1.3

- Fixed how to handle missing files in Pyxel Editor
- Fixed how to quit the application

## 1.1.2

- Fixed the way to decide the automatic screen size
- Fixed API descriptions in the README files
- Changed the way to handle runtime errors
- Changed the save and load functions to not return bool
- Specified the version of Python in the README files
- Added the play_pos function to Example 4
- Added the description of the included libraries to the README files
- Updated the screenshots of Examples 3 and 4
- Fixed game controller input
- Improved the performance of the tilemap editor

## 1.1.1

- Fixed the install_pyxel_examples script to include the .pyxres file

## 1.1.0

- Modified .gitignore to ignore .vscode files
- Changed to use SDL2 instead of GLFW
- Removed the refimg argument from the Tilemap methods
- Changed the save and load methods to return a bool
- Removed the run_with_profiler function
- Changed the max screen size to 256
- Added the play_pos function
- Changed arguments of the clip function
- Changed arguments of the rect and rectb functions
- Modified the examples according to the API changes
- Renamed the resource file extension to .pyxres
- Added the drop_file property
- Added the caption function
- Changed the way to detect the caller script

## 1.0.1

- Simplified comparisons
- Removed a deprecated function
- Applied @staticmethod to functions that do not use self
- Changed to keep the previous frame when cls is not called
- Changed frame_count to start from 0
- Fixed the gamepad constants

## 1.0.0

- Added an installation-method supplement to the documentation
- Added the type hints for Python

## 0.9.10

- Added image import instructions for Pyxel Editor to the README files
- Fixed the type hints
- Added GLFW dll for Windows

## 0.9.9

- Added the type hints for Python
- Added a description of the run_with_profiler function to the README files

## 0.9.8

- Added support to run Pyxel Editor without a filename

## 0.9.7

- Confirmed operation on Debian and Fedora
- Updated the installation instructions for Linux

## 0.9.6

- Updated the installation instructions for Linux
- Fixed a warning in setup.py

## 0.9.5

- Added issue templates
- Added a description of the issue templates to the README files
- Fixed the clipping bug of drawing primitives

## 0.9.4

- Fixed a crash when entering fullscreen mode
- Updated the description for Linux in the README files
- Reverted to check the version number of glfw strictly
- Increased the sound buffer size
- Fixed the range of the sound picker in the sound editor

## 0.9.3

- Added support to open a resource file by drag and drop in Pyxel Editor
- Renamed the constants for the mouse buttons
- Added gamepad support
- Changed gamepad available for Example 2
- Fixed the crash bug when the window is minimized
- Modified the code of Example 6
- Added the refimg property to the Tilemap class
- Removed the img argument from the bltm function
- Updated the screenshot of Example 3

## 0.9.2

- Added support to import PNG files by drag and drop in the image editor
- Fixed the crash bug caused by unsupported keys
- Added support to play the piano with the mouse in the sound editor
- Added support for repeating undo/redo shortcuts

## 0.9.1

- Fixed the color pick bug of the image editor
- Changed the focus UI of the image editor

## 0.9.0

- Fixed the bug where Pyxel Editor cursor malfunctions
- Added new function descriptions to Example 3
- Updated the screenshot of Example 3
- Stopped adding unnecessary undo history of the editors
- Added the setting files for Pipenv
- Modified the cursor design of the image/tilemap editor
- Added support to change the focus size of the tilemap editor
- Added the link to the subreddit in the README files
- Added support to export all constants for keys before init is called
- Added the contribution section to the README files

## 0.8.9

- Fixed the tilemap to allow the tiles of 256 or higher
- Updated the screenshots of Pyxel Editor
- Fixed the cursor movement of the sound/music editor
- Changed the caption of Example 2
- Renamed Example 6
- Fixed the error when saving long animated GIF

## 0.8.8

- Added the .pyxel file to the install example script

## 0.8.7

- Fixed the piano keyboard bug when only enter was pressed
- Fixed the piano keyboard highlights correctly
- Changed the default sound volume to 7
- Changed the default sound speed to 30
- Fixed the sound button bug of the sound editor

## 0.8.6

- Changed the operation method of the sound editor
- Changed to allow sound of length 0
- Implemented the music editor
- Changed Example 2 to use resource file

## 0.8.5

- Implemented the undo function of the sound editor
- Changed the click tolerance time
- Removed the length limitation of the sound
- Added the music and playm functions
- Changed Example 2 to use the music and playm functions

## 0.8.4

- Changed to follow the mouse position outside the window
- Changed to draw the self mouse cursor
- Added the mouse function
- Renamed arguments of the blt and bltm functions
- Reduced the size of PNG and animated GIF
- Changed the max length of the sound to 48
- Added the system option to the sound function
- Refined the help message of Pyxel Editor
- Added the ToggleButton and ImageToggleButton
- Implemented the sound editor except the undo function

## 0.8.3

- Fixed the right click bug in the tilemap editor
- Fixed the key callback bug
- Added the get method to the Image class
- Changed the set method of the Image class to accept a number as data
- Added the get and set methods to the Tilemap class
- Added the bltm test to Example 3
- Updated the descriptions of the README files

## 0.8.2

- Fixed the set, load, and copy methods of the image class
- Fixed the starting directory of the save and load functions
- Modified the usage of Pyxel Editor

## 0.8.1

- Added the run_with_profiler function
- Added the Tilemap class
- Added the bltm function
- Implemented the tilemap editor
- Added the standard widgets
- Changed the usage of Pyxel Editor
- Added the help messages to Pyxel Editor
- Added the description of Pyxel Editor to the README files

## 0.8.0

- Changed the formatter from yapf to black
- Added the UI module
- Added the save and load functions
- Added the image editor as a part of Pyxel Editor
- Removed the dirname option of the save and load functions

## 0.7.12

- Added new key definitions which integrates keys on both sides
- Changed the description of the project
- Changed the max screen size to 255
- Fixed the key hold time of the btnp function
- Fixed to work the btnp function correctly in the slow frame rate condition
- Changed the screen size of Example 5 and Example 6
- Updated the screenshot of Example 5

## 0.7.11

- Changed the Japanese link name in the README files
- Implemented the widget class for Pyxel Editor
- Added Example 6 by ttrkaya
- Renamed the constant variable for the max screen size
- Changed to set the default scale automatically

## 0.7.10

- Added screen size error
- Added the link to the Pyxel wiki to the README files
- Moved the README in Portuguese to the Pyxel wiki

## 0.7.9

- Changed the color of the system font texture to white
- Fixed a typo in the README.md
- Renamed the modules to use snake case
- Added glfw version check

## 0.7.8

- Added the system option to the image function
- Fixed the color bug of the shader for some environments
- Added the dirname option to the load method of the image class
- Updated the description of the init function in the README files

## 0.7.7

- Added the description of the screen size limitation to the README files
- Added the Fedora Linux installation to the README files
- Added another fallback to get the desktop folder name
- Changed the number of the image banks to 3
- Added some image assets for Pyxel Editor (WIP)

## 0.7.6

- Specified the GLFW version in the README files
- Limited the window size to 256 because of OpenGL Point Sprite limitation
- Fixed the element border lacks bug
- Added Example 5

## 0.7.5

- Fixed typos in the README files
- Updated the Arch Linux installation in the README files
- Updated the Debian Linux installation in the README files
- Updated the way to capture screen on Linux
- Fixed a shader compile error occurs in some environment

## 0.7.4

- Fixed to run without an audio card
- Refactored import order of all files with isort
- Fixed the way to capture screen on Linux

## 0.7.3

- Fixed the btnr function
- Fixed a typo in the README.md
- Added the title logo to the README files
- Added the Portuguese version of the README.md

## 0.7.2

- Changed to not include the screenshots in the PyPI package
- Removed unnecessary semicolons in the shader to avoid compile errors
- Changed the project description for PyPI
- Added installation instructions for Linux to the README files
- Refactored the way to make a captured image and animation
- Updated the screenshots of Example 3 and Example 4

## 0.7.1

- Modified the bgm of Example 2
- Renamed the argument 'no' of the image-related methods to 'img'
- Renamed the argument 'no' of the sound-related methods to 'snd'
- Fixed to include the assets and screenshots in the PyPI package

## 0.7.0

- Modified Example 1 to use the App class
- Renamed and modified Example 2
- Removed the logo function and added the logo image
- Improved the performance of the text function
- Updated the README.md and README.ja.md
- Replaced Example 2
- Removed the resize method of the Image class
- Changed the size of the Image to 256x256
- Fixed the copy method of the Image class

## 0.6.0

- Changed the properties of the Sound class to public
- Added offset arguments to the Image load method
- Added the copy method to the Image class
- Renamed arguments of the image and sound functions
- Added the window icon
- Added the logo function
- Added the resize method to the Image class
- Refined Example 1-4

## 0.5.0

- Added the version number constant
- Renamed the examples copy script to install_pyxel_examples
- Removed unnecessary scripts
- Separated the constant definitions
- Added image functions and renamed related functions
- Added sound functions and renamed related functions

## 0.4.0

- Changed the key assigns of the special inputs
- Added the screen capture functions (still image and video)
- Included the examples in the package and added the copy script
- Added the fromstring method to the Image class
- Added the fromstring method to the Sound class

## 0.3.0

- Added the '-'(flat) syntax to the Sound class
- Added the set method to the Image class again
- Renamed the track to channel
- Changed the play function to enable playing a sound list

## 0.2.0

- Added the audio playback function
- Removed the set method of the Image class

## 0.1.0

- First alpha release
