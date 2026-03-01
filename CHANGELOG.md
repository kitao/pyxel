# Change Log

## 2.7.3

- Added KMSDRM video driver for embedded Linux devices
- Added MCP server for AI-assisted development
- Added VS Code extension
- Rewrote examples 06 and 07 for consistency
- Expanded Rust unit tests across 10 modules
- Improved error handling and unified error messages

## 2.7.2

- Revamped FAQ with updated content and improved clarity
- Revamped Pyxel Web documentation for clarity and accuracy
- Removed version number from pyxel command example in READMEs
- Added manual page for Pyxel MML Studio
- Added manual page for Pyxel Code Maker

## 2.7.1

- Removed keyword-only separators from pyi and PyO3 bindings
- Fixed type hints in pyi for Seq, Tilemap, Sound, and Music
- Added API Reference page for Pyxel Web
- Improved web tools with i18n support and other enhancements
- Redesigned Pyxel Examples page as Pyxel Showcase

## 2.7.0

- Refactored imports and cleanup across Rust, Python, and JS
- Improved test script with cleanup and app2exe/app2html
- Renamed pyxel-core public API to idiomatic Rust names
- Optimized drawing performance with bulk fills and reduced overhead
- Renamed internal Python module to pyxel_binding
- Removed mutexes for resource types to improve performance

## 2.6.9

- Fixed Linux SDL2 build to include X11/Wayland/ALSA/PulseAudio drivers
- Added Python list-like operations to sequence types
- Added pre-release versioning support for PyPI
- Removed crates.io publishing from release workflow
- Restructured project layout and reorganized Rust crates

## 2.6.8

- Updated Linux CI SDL2 version to 2.32.0
- Renamed SDL2 feature flags to sdl2_system and sdl2_bundle
- Bundled SDL2 in Linux wheels for easier installation

## 2.6.7

- Removed version update check from the pyxel command
- Fixed keyboard input for non-US layouts on the web version
- Updated Pyxel thanks image
- Updated CI Python version to 3.14
- Synced CI Rust version with rust-toolchain.toml
- Renamed and simplified tools scripts
- Refined the Makefile for safer WASM builds

## 2.6.6

- Updated zip crate to version 8.1
- Updated toml crate to version 1.0
- Added BGM generation feature to Example 15
- Removed the transp argument from gen_bgm
- Fixed gen_bgm parity and seed determinism
- Reduced PCM load with fixed-point mixing
- Reduced synth load with fixed-point amplitude mixing

## 2.6.5

- Updated rand_xoshiro crate to version 0.8
- Updated rand crate to version 0.10
- Updated sgn to preserve input types in Python
- Updated Examples 10 and 15 to use collide
- Added collide method to Tilemap
- Added clamp function
- Added gen_bgm for automatic BGM generation and playback
- Removed unnecessary files to reduce the wheel size

## 2.6.4

- Removed unnecessary files to reduce the wheel size

## 2.6.3

- Renamed incl_colors to include_colors in Image
- Added Example 18 for audio playback
- Added pcm method to Sound for audio playback
- Enabled local Pyxel for the test web server
- Normalized HTML doctypes to lowercase
- Adjusted initialization order for the web version

## 2.6.2

- Fixed loading of additional files in Pyxel Code Maker
- Disabled pinch/double-tap zoom on mobile browsers
- Updated pyo3 crate to version 0.28
- Updated Pyodide to version 0.29.3

## 2.6.1

- Updated sysinfo crate to version 0.38
- Updated Pyodide to version 0.29.2
- Increased audio buffer size to 1024 for the web version

## 2.6.0

- Updated Example 13 to demonstrate custom font rendering
- Added font rendering feature using TTF and OTF fonts

## 2.5.13

- Added links to tool manuals in the README files
- Added slur (legato) support to MML
- Fixed memory leak in MML parser

## 2.5.12

- Updated sysinfo crate to version 0.37
- Updated image crate to version 0.25
- Updated Rust to version nightly-2025-12-10
- Updated Pyodide to version 0.29.1
- Renamed scripts directory to tools
- Refined the README files
- Fixed double slashes in README-abspath URLs

## 2.5.11

- Downgraded the image crate to version 0.24
- Updated quit function behavior in the web version
- Updated zip crate to version 7.0
- Updated gif crate to version 0.14
- Improved error handling for the web version
- Updated web version to display all errors

## 2.5.10

- Added Pyxel Code Maker zip file support to the play command
- Added Pyxel Code Maker web page
- Updated pyo3 crate to version 0.27
- Updated Pyodide to version 0.29.0

## 2.5.9

- Updated zip crate to version 6.0
- Updated web pages to refer to the main branch
- Updated SDL2 to version 2.32.0
- Updated Emscripten to version 4.0.9
- Updated Pyodide to version 0.28.3
- Enabled automatic color picker size adjustment in Pyxel Editor
- Enabled palette file download for new files in Pyxel Editor
- Added load_pal and save_pal functions

## 2.5.8

- Updated build environment version for Mac to macOS 15
- Fixed Tilemap.data_ptr to expose full map data
- Set desktop OpenGL internal format to GL_R8

## 2.5.7

- Enabled parent HTML window to control the initial input wait
- Improved usability of Pyxel MML Studio
- Updated URL on reload in Pyxel MML Studio
- Added links to web tools and examples for Pyxel in the README files
- Specified Tailwind CSS version 3.4.17 for Pyxel web pages

## 2.5.6

- Updated design of the Pyxel web pages
- Improved Pyxel MML Studio usability
- Updated Pyxel MML Studio to use compressed URLs

## 2.5.5

- Separated the web MML commands into the Pyxel MML Studio page
- Updated tone selection to use 0 when a non-existent tone number is specified
- Adjusted error output display size in the web version
- Improved automatic file download for the web version
- Removed extra directories after the app2exe command
- Reworked reset function and play command behavior
- Fixed touch device detection for Firefox in the web version
- Updated exe packaging for the reset function with PyInstaller

## 2.5.4

- Updated zip crate to version 5.0
- Updated design of the web pages
- Fixed reset function issue when called inside pyxapp
- Fixed Example 17 Python command execution issue
- Added two Pyxel apps by Adam for the app launcher

## 2.5.3

- Added START and BACK buttons to the virtual gamepad for the web
- Excluded GIF and ZIP files from Pyxel application files
- Added a gamepad shortcut for the reset operation
- Updated HTML pages to use the latest Pyxel from CDN

## 2.5.2

- Added gamepad support to Example 15
- Updated pyo3 crate to version 0.26
- Added Example 17 for the app launcher and the reset function
- Added three sample games from the Pyxel book
- Added an environment variable for the reset function's window state
- Fixed cargo publish error by adding features sdl2_bundle

## 2.5.1

- Fixed app2exe issue with white spaces
- Added line break support for custom font rendering
- Fixed cargo publish error by adding features sdl2
- Updated reset to preserve environment variables

## 2.5.0

- Fixed delayed sound playback on Android browsers
- Added automatic use of old_mml when '~' is used
- Added reset function
- Reduced error output in the web version
- Refactored the platform abstraction layer

## 2.4.10

- Fixed parameter commands ignored after repeat in MML

## 2.4.9

- Added support for tie notation with numbers only in MML
- Fixed dot note length bug in MML parser

## 2.4.8

- Added console output to the mml command in Pyxel Web Launcher
- Fixed playback when all sounds in the array are empty

## 2.4.7

- Fixed a vibrato bug when the sound speed is low

## 2.4.6

- Updated sysinfo crate to version 0.36
- Updated web usage instructions
- Pinned the Pyxel version used by the app2html command
- Set note interpolation time to 1 ms
- Added mml command to Pyxel Web Launcher

## 2.4.5

- Added call to old_mml method when the old syntax is detected

## 2.4.4

- Updated Pyodide to version 0.27.7
- Updated toml crate to version 0.9
- Cleaned up and improved usability of Example 14
- Added documentation on pinning the Pyxel version for the web version
- Restored tick option of the play and playm functions
- Fixed a cargo login warning

## 2.4.3

- Restored excl options in the load and save functions
- Added note interpolation processing to suppress click noise

## 2.4.2

- Reverted the add_delta in blip_buf to prevent audio degradation

## 2.4.1

- Removed redundant MML code from Example 9
- Added asterisk parameter support to the @GLI command in MML
- Switched to the blip_buf crate
- Updated Sound member types
- Renamed tone_index parameter of the Tone command in MML to tone
- Made the wavetable field of Tone support arbitrary length
- Added sample_bits field to Tone
- Renamed noise field of Tone to mode

## 2.4.0

- Updated default floating-point type to f32
- Updated Example 9 to use new MML syntax
- Removed incl options from the load and save functions
- Renamed excl options to exclude in the load and save functions
- Added MML string support to the play method of Channel
- Added MML string support to the play function
- Added old_mml method to Sound for the old MML syntax
- Updated mml method of Sound to use the new MML syntax
- Added total_sec method to Sound
- Updated Sound and Music save methods to use sec instead of count
- Updated play_pos to return sec instead of note_no
- Updated play and playm to use sec instead of tick
- Renamed waveform field of Tone to wavetable
- Renewed the sound engine and MML syntax
- Fixed GitHub Actions to use Rust nightly-2025-02-01
- Updated bindgen crate to version 0.72
- Updated sysinfo crate to version 0.35
- Updated pyo3 crate to version 0.25
- Updated serde-xml-rs to version 0.8
- Updated zip crate to version 4.0
- Added a Q&A about saving application data to the FAQ
- Updated Pyodide to version 0.27.5
- Fixed audio module initialize arguments
- Removed colors, tones, and channels from the resource format

## 2.3.18

- Updated Pyxel thanks image
- Added DEFAULT_COLORS constant
- Updated math functions to be static
- Updated sysinfo crate to version 0.34
- Added blank lines to format code
- Added a screen size specification to the README files
- Updated message image
- Fixed relative path handling in the watch command
- Added SDL2 include paths for Linux

## 2.3.17

- Removed autoplay code from the web version
- Updated btn-related functions to use assert

## 2.3.16

- Modified the audio resume processing for the web version

## 2.3.15

- Fixed audio resuming in the web version
- Organized the FAQ section

## 2.3.14

- Updated image crate to version 0.25
- Added version number output on startup in the web version
- Added warnings for invalid keys in btn-related functions

## 2.3.13

- Fixed a bug that broke the app2html command
- Modified the inclusion order of SDL.h

## 2.3.12

- Updated pyo3 crate to version 2.4
- Added support for overriding screen position and size in the web version
- Added a Q&A about file loading to the FAQ

## 2.3.11

- Updated file download check in the web version
- Removed Google Analytics links from the web pages

## 2.3.10

- Handled XMLHttpRequest exceptions in the web version
- Fixed stack overflow issue in the fill function

## 2.3.9

- Fixed a warning in a utility script
- Stopped downloading unnecessary files in the web version
- Updated Pyodide to version 0.27.3
- Adjusted click message removal timing in the web version
- Added support for loading upper-level files in the web version

## 2.3.8

- Updated usage instructions for the web version
- Fixed local module imports in the web version

## 2.3.7

- Updated Rust to version nightly-2025-02-01
- Added support for local module imports in the web version
- Added a script for Pyxel User Examples pages

## 2.3.6

- Updated blt and bltm rotation to be clockwise

## 2.3.5

- Stopped using the once_cell crate
- Fixed input_text variable
- Aligned Emscripten version with Pyodide
- Fixed text function ignoring camera when font set
- Fixed multi-gamepad support
- Updated 8bit BGM generator to version 1.30
- Updated rand_xoshiro crate to version 0.7
- Updated rand crate to version 0.9
- Updated GitHub Action scripts

## 2.3.4

- Downgraded Pyodide to version 0.27.0

## 2.3.3

- Restored links to the Discord servers in the README files
- Moved the image used for MP4 creation
- Fixed a bug in the save method of Music
- Removed NoSleep.js from the web version of Pyxel

## 2.3.2

- Added save method to the Sound and Music classes
- Added hound crate

## 2.3.1

- Updated 8bit BGM generator to version 1.22
- Updated Example 9 to use MML for music setup
- Restored input_keys variable
- Added support for adding multiple dots to a note in MML
- Renamed `!` command in MML to `~`
- Added `X` command to MML
- Fixed a bug in loading old resource files

## 2.3.0

- Updated year in the LICENSE files
- Updated Pyodide to version 0.27.1
- Reduced click noise
- Updated sample rate to 22.05 kHz
- Updated directories crate to version 6.0
- Added mml method to Sound
- Adjusted size of sound-related types
- Added ToneIndex type

## 2.2.11

- Formatted sound strings in examples
- Fixed an input issue in the sound editor
- Fixed types in the pyi file

## 2.2.10

- Reduced sound clock rate from 120MHz to 2.048MHz
- Updated pyo3 crate to version 2.3

## 2.2.9

- Fixed clippy warnings
- Updated message image for the README files
- Excluded the pycache directory from the copy_examples command
- Added LICENSE file to the Python package

## 2.2.8

- Fixed a bug in the mouse cursor position
- Raised the minimum supported macOS to version 13
- Updated bindgen crate to version 0.71
- Updated glow crate to version 0.16
- Updated sysinfo crate to version 0.33
- Updated once_cell crate to version 1.20
- Updated indexmap crate to version 2.7
- Updated Pyodide to version 0.26.4
- Updated Maturin to the latest version
- Updated Tilemap editor to load Layer 0 when a TMX file is dropped onto it
- Replaced the usage of a deprecated API
- Renamed (tile_x, tile_y) to (image_tx, image_ty) in the README files
- Modified a shortcut description in the README files

## 2.2.7

- Updated default scaling to maximum
- Added gamepad shortcuts using A+B+X+Y+DL/DR/DU/DD
- Added integer-scale toggle feature with Alt(Option)+8
- Renamed argument of the fullscreen function
- Added integer_scale function
- Added perf_monitor function
- Updated Pyodide to version 0.26.3

## 2.2.6

- Fully revised the translations of all README files
- Updated glow crate to version 0.15
- Switched from the platform-dir crate to the directories crate
- Added user_data_dir function
- Renamed WORKING_DIR to BASE_DIR

## 2.2.5

- Added a note regarding the usage of the run command on the web
- Fixed displayed color issue caused by the sRGB setting

## 2.2.4

- Updated Q&A
- Updated instructions for using the web version of Pyxel
- Updated license description in the README files
- Updated sysinfo crate to version 0.25
- Fixed a bug when playing a pyxapp with the same process ID

## 2.2.3

- Fixed new clippy warnings
- Fixed a warning on macOS Sonoma
- Added pyxel.cli.print_pyxel_app_metadata function
- Added pyxel.cli.get_pyxel_app_metadata function
- Ensured that the metadata is in UTF-8 format
- Updated description of Pyxel's features in the README files

## 2.2.2

- Added metadata to the bundled Pyxel application files
- Added support for adding metadata to a Pyxel application file
- Updated Python in GitHub Actions to version 3.12
- Updated an image layout in the README files
- Fixed mypy errors

## 2.2.1

- Updated Example 14 to use native font rendering
- Added a font option to the text function
- Added Font class
- Removed an unnecessary line in Example 14
- Added watch command description to the README files

## 2.2.0

- Added Example 16 for rotation and scaling
- Modified the function notation in Example 4
- Specified Maturin to version 1.7.0 to prevent linking errors
- Added rotate and scale options to the blt and bltm functions
- Removed keyword-only arguments

## 2.1.10

- Updated bindgen crate to version 0.70
- Replaced links to Twitter with X in the README files
- Fixed a color rendering issue on Windows

## 2.1.9

- Fixed key state changes during special inputs
- Avoided using the gil-refs feature in the pyo3 crate
- Updated required Python to version 3.8 or higher

## 2.1.8

- Fixed keyword-only arguments functionality
- Updated pyo3 crate to version 0.22
- Updated SDL2 to version 2.28.4
- Updated Emscripten to version 3.1.61

## 2.1.7

- Updated sysinfo crate to version 0.31
- Fixed incorrect array references during playback in Pyxel Editor
- Prevented editing during playback in Pyxel Editor
- Fixed push back process in Example 10 and 15
- Updated sgn to return integer
- Modified help messages in Pyxel Editor

## 2.1.6

- Updated Pyodide to version 0.26.2
- Updated glow crate to version 0.14
- Fixed a warning on macOS Sonoma
- Added Turkish and Ukrainian README files
- Updated message image for the README files

## 2.1.5

- Updated mutex control for sound playback
- Suppressed the outdated resource file version warning
- Updated initial value for the noise sound register
- Added a value change shortcut to the sound and music editors
- Updated description of the set_effects method in the README files

## 2.1.4

- Fixed version check for the resource file
- Added a bank copy feature to Pyxel Editor

## 2.1.3

- Incremented the resource format version

## 2.1.2

- Fixed warp_mouse function
- Added Half-FadeOut and Quarter-FadeOut effects to Sound
- Updated descriptions of the pget and pset functions

## 2.1.1

- Updated zip crate to a newer version
- Removed non-functional CTRL+Drop feature from Pyxel Editor
- Fixed resume option of the play function

## 2.1.0

- Updated Pyodide to version 0.26.1
- Added descriptions of the resume option to the README files
- Updated Example 9 to use the resume option for SFX playback
- Updated function notation in Example 4
- Added a resume option to the play function
- Fixed a help message in Pyxel Editor

## 2.0.14

- Fixed bltm referencing out of range

## 2.0.13

- Fixed app2exe and app2html commands
- Updated make update command

## 2.0.12

- Updated Emscripten to version 3.1.53
- Updated Pyodide to version 0.25.1
- Modified build instructions in Makefile
- Fixed installation instructions for Mac in the README files

## 2.0.11

- Updated crate versions
- Added error messages for the pyxel command

## 2.0.10

- Added support for encodings other than UTF-8 in the app2exe command
- Fixed a non pixel perfect bug for OpenGL ES
- Fixed sqrt function
- Moved pyproject.toml and requirements.txt

## 2.0.9

- Fixed release script
- Fixed timing to disable the slide effect

## 2.0.8

- Fixed clippy warnings
- Disabled slide effect on the first note of a sound
- Organized project metadata for Rust and Python
- Updated project directory structure

## 2.0.7

- Fixed a color count change bug on OpenGL ES
- Refined Example 10
- Added Example 15
- Added an incl_colors option to the load method of Image
- Added an incl_colors option to the from_image method of Image
- Refined the code for Example 9
- Updated destination for image drag-and-drop in the image editor
- Enabled importing a TMX file via drag and drop in the tilemap editor
- Added load_tmx and load method to Tilemap
- Added a shortcut to output the current color palette
- Turned off the high DPI mode for performance perspective

## 2.0.6

- Updated how OpenGL vs OpenGL ES is selected
- Added support for high DPI mode

## 2.0.5

- Fixed a bug in Pyxel Editor when creating new resource files
- Restored publish of the crate to the release script

## 2.0.4

- Added a shortcut to output an image bank
- Fixed a mouse wheel bug

## 2.0.3

- Updated mouse cursor position when focus is lost
- Increased audio clock rate to 120MHz

## 2.0.2

- Added Example 14
- Added usage of the show and flip functions to the README files
- Switched to Ruff for linting and formatting Python code
- Music.set no longer requires specifying all channels
- Reordered declarations in the pyi file

## 2.0.1

- Updated CDN links to use the latest Pyxel explicitly
- Updated resource file format for the tones
- Renamed Waveform and waveforms to Tone and tones
- Removed source code path from the binary
- Tile coordinate type is back to u8
- Removed publish of the crate to the release script

## 2.0.0

- Updated 8bit BGM generator to the latest version
- Added a detune field to Channel
- Added waveforms as a system list
- Added Waveform class for waveform editing
- Updated arguments for the load and save functions
- Switched to a new resource format based on TOML
- Marked the snds_list of Music as a deprecated field
- Renamed snds_list of Music to seqs
- Marked the image and refimg of Tilemap as deprecated fields
- Integrated the image and refimg of Tilemap into imgsrc
- Removed set_btn and set_btnv functions
- Removed input_keys variable
- Removed is_fullscreen variable
- Renamed drop_files variable to dropped_files
- Renamed set_mouse_pos function to warp_mouse
- Renamed reset_capture function to reset_screencast
- Marked the channel, sounds, and music functions as deprecated functions
- Added channels, sounds, and musics as system lists
- Marked the image and tilemap functions as deprecated functions
- Added images, tilemaps as system lists
- Added dither function to set dithering type
- Added support for the third and fourth gamepads
- Added a shortcut to change the screen mode with Alt(Option)+9
- Added screen_mode function to change screen rendering type
- Updated SDL2 to version 2.24.2
- Updated Emscripten to version 3.1.45
- Updated Pyodide to version 0.24.1
- Switched screen rendering to GLSL
- Added support for resizing the colors list
- Switched to the C version of SDL2
