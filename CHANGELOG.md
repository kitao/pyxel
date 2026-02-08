# Change Log

## 2.6.4

- Removed unnecessary files to reduce the wheel size
- Added gen_bgm for automatic BGM generation and playback

## 2.6.3

- Adjusted initialization order for the web version
- Normalized HTML doctypes to lowercase
- Enabled local Pyxel for the test web server
- Added pcm method to the Sound class for audio playback
- Added Example 18 for audio playback
- Renamed incl_colors to include_colors in the Image class

## 2.6.2

- Updated Pyodide to version 0.29.3
- Updated pyo3 crate to version 0.28
- Disabled pinch/double-tap zoom on mobile browsers
- Fixed loading of additional files in Pyxel Code Maker

## 2.6.1

- Increased audio buffer size to 1024 for the web version
- Updated Pyodide to version 0.29.2
- Updated sysinfo crate to version 0.38

## 2.6.0

- Added font rendering feature using TTF and OTF fonts
- Updated Example 13 to demonstrate custom font rendering

## 2.5.13

- Fixed memory leak in MML parser
- Added slur (legato) support to MML
- Added links to tool manuals in the README files

## 2.5.12

- Fixed double slashes in README-abspath URLs
- Refined the README files
- Renamed scripts directory to tools
- Updated Pyodide to version 0.29.1
- Updated Rust to version nightly-2025-12-10
- Updated image crate to version 0.25
- Updated sysinfo crate to version 0.37

## 2.5.11

- Web version now displays all errors
- Improved error handling for the web version
- Updated gif crate to version 0.14
- Updated zip crate to version 7.0
- Updated quit function behavior in the web version
- Downgraded the image crate to version 0.24

## 2.5.10

- Updated Pyodide to version 0.29.0
- Updated pyo3 crate to version 0.27
- Added Pyxel Code Maker web page
- Added Pyxel Code Maker zip file support to the play command

## 2.5.9

- Added load_pal and save_pal functions
- Enabled palette file download for new files in Pyxel Editor
- Enabled automatic color picker size adjustment in Pyxel Editor
- Updated Emscripten to version 4.0.9
- Updated Pyodide to version 0.28.3
- Updated SDL2 to version 2.32.0
- Web pages now refer to the main branch
- Updated zip crate to version 6.0

## 2.5.8

- Set desktop OpenGL internal format to GL_R8
- Fixed Tilemap.data_ptr to expose full map data
- Updated build environment version for Mac to macOS 15

## 2.5.7

- Specified Tailwind CSS version 3.4.17 for Pyxel web pages
- Added links to web tools and examples for Pyxel in the README files
- Updated URL on reload in Pyxel MML Studio
- Improved usability of Pyxel MML Studio
- Enabled parent HTML window to control the initial input wait

## 2.5.6

- Pyxel MML Studio now uses compressed URLs
- Improved Pyxel MML Studio usability
- Updated design of the Pyxel web pages

## 2.5.5

- Updated exe packaging for the reset function with PyInstaller
- Fixed touch device detection for Firefox in the web version
- Reworked reset function and play command behavior
- Removed extra directories after the app2exe command
- Improved automatic file download for the web version
- Adjusted error output display size in the web version
- Updated reset function mechanism
- Now uses tone 0 when a non-existent tone number is specified
- Separated the web MML commands into the Pyxel MML Studio page

## 2.5.4

- Added two Pyxel apps by Adam for the app launcher
- Fixed Example 17 Python command execution issue
- Fixed reset function issue when called inside pyxapp
- Updated design of the web pages
- Updated zip crate to version 5.0

## 2.5.3

- HTML pages now use the latest Pyxel from CDN
- Added a gamepad shortcut for the reset operation
- Excluded GIF and ZIP files from Pyxel application files
- Added START and BACK buttons to the virtual gamepad for the web

## 2.5.2

- Fixed cargo publish error by adding features sdl2_bundle
- Added an environment variable for the reset function's window state
- Added three sample games from the Pyxel book
- Added Example 17 for the app launcher and the reset function
- Updated pyo3 crate to version 0.26
- Added gamepad support to Example 15

## 2.5.1

- Now preserves environment variables on reset
- Fixed cargo publish error by adding features sdl2
- Added line break support for custom font rendering
- Fixed app2exe issue with white spaces

## 2.5.0

- Refactored the platform abstraction layer
- Reduced error output in the web version
- Added reset function
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

- Added mml command to Pyxel Web Launcher
- Set note interpolation time to 1 ms
- Pinned the Pyxel version used by the app2html command
- Updated web usage instructions
- Updated sysinfo crate to version 0.36

## 2.4.5

- Added call to old_mml method when the old syntax is detected

## 2.4.4

- Fixed a cargo login warning
- Restored tick option of the play and playm functions
- Added documentation on pinning the Pyxel version for the web version
- Cleaned up and improved usability of Example 14
- Updated toml crate to version 0.9
- Updated Pyodide to version 0.27.7

## 2.4.3

- Added note interpolation processing to suppress click noise
- Restored excl options in the load and save functions

## 2.4.2

- Reverted the add_delta in blip_buf to prevent audio degradation

## 2.4.1

- Renamed noise field of the Tone class to mode
- Added sample_bits field to the Tone class
- Made the wavetable field of the Tone class support arbitrary length
- Renamed tone_index parameter of the Tone command in MML to tone
- Updated Sound class member types
- Switched to the blip_buf crate
- Added asterisk parameter support to the @GLI command in MML
- Removed redundant MML code from Example 9

## 2.4.0

- Fixed audio module initialize arguments
- Updated Pyodide to version 0.27.5
- Added a Q&A about saving application data to the FAQ
- Updated zip crate to version 4.0
- Updated serde-xml-rs to version 0.8
- Updated pyo3 crate to version 0.25
- Updated sysinfo crate to version 0.35
- Updated bindgen crate to version 0.72
- Fixed GitHub Actions to use Rust nightly-2025-02-01
- Renewed the sound engine and MML syntax
- Renamed waveform field of the Tone class to wavetable
- play and playm now use sec instead of tick
- play_pos now returns sec instead of note_no
- Sound and Music save methods now use sec instead of count
- Added total_sec method to the Sound class
- mml method of the Sound class now uses the new MML syntax
- Added old_mml method to the Sound class for the old MML syntax
- Added MML string support to the play function
- Added MML string support to the play method of the Channel class
- Removed colors, tones, and channels from the resource format
- Renamed excl options to exclude in the load and save functions
- Removed incl options from the load and save functions
- Updated Example 9 to use new MML syntax
- Default floating-point type is now f32

## 2.3.18

- Added SDL2 include paths for Linux
- Fixed relative path handling in the watch command
- Updated message image
- Added a screen size specification to the README files
- Added blank lines to format code
- Updated sysinfo crate to version 0.34
- Math functions are now static
- Added DEFAULT_COLORS constant
- Updated Pyxel thanks image

## 2.3.17

- btn-related functions now use assert
- Removed autoplay code from the web version

## 2.3.16

- Modified the audio resume processing for the web version

## 2.3.15

- Organized the FAQ section
- Fixed audio resuming in the web version

## 2.3.14

- Added warnings for invalid keys in btn-related functions
- Added version number output on startup in the web version
- Updated image crate to version 0.25

## 2.3.13

- Modified the inclusion order of SDL.h
- Fixed a bug that broke the app2html command

## 2.3.12

- Added a Q&A about file loading to the FAQ
- Added support for overriding screen position and size in the web version
- Updated pyo3 crate to version 2.4

## 2.3.11

- Removed Google Analytics links from the web pages
- Updated file download check in the web version

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
- Updated usage instructions for the web version

## 2.3.7

- Added a script for Pyxel User Examples pages
- Added support for local module imports in the web version
- Updated Rust to version nightly-2025-02-01

## 2.3.6

- Rotation in blt and bltm is now clockwise

## 2.3.5

- Updated GitHub Action scripts
- Updated rand crate to version 0.9
- Updated rand_xoshiro to version 0.7
- Updated 8bit BGM Generator to version 1.30
- Fixed multi-gamepad support
- Fixed text function ignoring camera when font set
- Aligned Emscripten version with Pyodide
- Fixed input_text variable
- Stopped using the once_cell crate

## 2.3.4

- Downgraded Pyodide to version 0.27.0

## 2.3.3

- Removed NoSleep.js from the web version of Pyxel
- Fixed a bug in the save method of the Music class
- Moved the image used for MP4 creation
- Restored links to the Discord servers in the README files

## 2.3.2

- Added hound crate
- Added save method to the Sound and Music classes

## 2.3.1

- Fixed a bug in loading old resource files
- Added `X` command to MML
- Renamed `!` command in MML to `~`
- Added support for adding multiple dots to a note in MML
- Restored input_keys variable
- Updated Example 9 to use MML for music setup
- Updated 8bit BGM Generator to version 1.22

## 2.3.0

- Added ToneIndex type
- Adjusted size of sound-related types
- Added mml method to the Sound class
- Updated directories crate to version 6.0
- Sample rate is now 22.05 kHz
- Reduced click noise
- Updated Pyodide to version 0.27.1
- Updated year in the LICENSE files

## 2.2.11

- Fixed types in the pyi file
- Fixed an input issue in the sound editor
- Formatted sound strings in examples

## 2.2.10

- Updated pyo3 crate to version 2.3
- Reduced sound clock rate from 120MHz to 2.048MHz

## 2.2.9

- Added LICENSE file to the Python package
- Excluded the pycache directory from the copy_examples command
- Updated message image for the README files
- Fixed clippy warnings

## 2.2.8

- Modified a shortcut description in the README files
- Renamed (tile_x, tile_y) to (image_tx, image_ty) in the README files
- Replaced the usage of a deprecated API
- Tilemap editor now loads Layer 0 when a TMX file is dropped onto it
- Updated Maturin to the latest version
- Updated Pyodide to version 0.26.4
- Updated indexmap crate to version 2.7
- Updated once_cell crate to version 1.20
- Updated zip crate to version 2.2
- Updated sysinfo crate to version 0.33
- Updated glow crate to version 0.16
- Updated bindgen crate to version 0.71
- Raised the minimum supported macOS to version 13
- Fixed a bug in the mouse cursor position

## 2.2.7

- Updated Pyodide to version 0.26.3
- Added perf_monitor function
- Added integer_scale function
- Renamed argument of the fullscreen function
- Added integer-scale toggle feature with Alt(Option)+8
- Added gamepad shortcuts using A+B+X+Y+DL/DR/DU/DD
- Default scaling is now maximum

## 2.2.6

- Renamed WORKING_DIR to BASE_DIR
- Added user_data_dir function
- Switched from the platform-dir crate to the directories crate
- Updated glow crate to version 0.15
- Fully revised the translations of all README files

## 2.2.5

- Fixed displayed color issue caused by the sRGB setting
- Added a note regarding the usage of the run command on the web

## 2.2.4

- Fixed a bug when playing a pyxapp with the same process ID
- Updated sysinfo crate to version 0.25
- Updated license description in the README files
- Updated instructions for using the web version of Pyxel
- Updated Q&A

## 2.2.3

- Updated description of Pyxel's features in the README files
- Ensured that the metadata is in UTF-8 format
- Added pyxel.cli.get_pyxel_app_metadata function
- Added pyxel.cli.print_pyxel_app_metadata function
- Fixed a warning on macOS Sonoma
- Fixed new clippy warnings

## 2.2.2

- Fixed mypy errors
- Updated an image layout in the README files
- Updated Python in GitHub Actions to version 3.12
- Added support for adding metadata to a Pyxel application file
- Added metadata to the bundled Pyxel application files

## 2.2.1

- Added watch command description to the README files
- Removed an unnecessary line in Example 14
- Added Font class
- Added a font option to the text function
- Example 14 now uses native font rendering

## 2.2.0

- Removed keyword-only arguments
- Added rotate and scale options to the blt and bltm functions
- Specified Maturin to version 1.7.0 to prevent linking errors
- Modified the function notation in Example 4
- Added Example 16 for rotation and scaling

## 2.1.10

- Fixed a color rendering issue on Windows
- Replaced links to Twitter with X in the README files
- Updated bindgen crate to version 0.70

## 2.1.9

- Updated required Python to version 3.8 or higher
- Avoided using the gil-refs feature in the pyo3 crate
- Fixed key state changes during special inputs

## 2.1.8

- Updated Emscripten to version 3.1.61
- Updated SDL2 to version 2.28.4
- Updated pyo3 crate to version 0.22
- Fixed keyword-only arguments functionality

## 2.1.7

- Modified help messages in Pyxel Editor
- sgn now returns integer
- Fixed push back process in Example 10 and 15
- Prevented editing during playback in Pyxel Editor
- Fixed incorrect array references during playback in Pyxel Editor
- Updated sysinfo crate to version 0.31

## 2.1.6

- Updated message image for the README files
- Added Turkish and Ukrainian README files
- Fixed a warning on macOS Sonoma
- Updated Pyodide to version 0.26.2
- Updated glow crate to version 0.14

## 2.1.5

- Updated description of the set_effects method in the README files
- Added a value change shortcut to the sound and music editors
- Updated initial value for the noise sound register
- Suppressed the outdated resource file version warning
- Updated mutex control for sound playback

## 2.1.4

- Added a bank copy feature to Pyxel Editor
- Fixed version check for the resource file

## 2.1.3

- Incremented the resource format version

## 2.1.2

- Updated descriptions of the pget and pset functions
- Added Half-FadeOut and Quarter-FadeOut effects to the Sound class
- Fixed warp_mouse function

## 2.1.1

- Fixed resume option of the play function
- Removed non-functional CTRL+Drop feature from Pyxel Editor
- Updated zip crate version

## 2.1.0

- Fixed a help message in Pyxel Editor
- Added a resume option to the play function
- Updated function notation in Example 4
- Example 9 now uses the resume option for SFX playback
- Added descriptions of the resume option to the README files
- Updated Pyodide to version 0.26.1

## 2.0.14

- Fixed bltm referencing out of range

## 2.0.13

- Updated make update command
- Fixed app2exe and app2html commands

## 2.0.12

- Fixed installation instructions for Mac in the README files
- Modified build instructions in Makefile
- Updated Pyodide to version 0.25.1
- Updated Emscripten to version 3.1.53

## 2.0.11

- Added error messages for the pyxel command
- Updated crate versions

## 2.0.10

- Moved pyproject.toml and requirements.txt
- Fixed sqrt function
- Fixed a non pixel perfect bug for OpenGL ES
- Added support for encodings other than UTF-8 in the app2exe command

## 2.0.9

- Fixed timing to disable the slide effect
- Fixed release script

## 2.0.8

- Updated project directory structure
- Organized project metadata for Rust and Python
- Disabled slide effect on the first note of a sound
- Fixed clippy warnings

## 2.0.7

- Turned off the high DPI mode for performance perspective
- Added a shortcut to output the current color palette
- Added load_tmx and load method to the Tilemap class
- Enabled importing a TMX file via drag and drop in the tilemap editor
- Updated destination for image drag-and-drop in the image editor
- Refined the code for Example 9 and Example 10
- Added an incl_colors option to the from_image method of the Image class
- Added an incl_colors option to the load method of the Image class
- Added Example 15
- Refined Example 10
- Fixed a color count change bug on OpenGL ES

## 2.0.6

- Added support for high DPI mode
- Updated how OpenGL vs OpenGL ES is selected

## 2.0.5

- Restored publish of the crate to the release script
- Fixed a bug in Pyxel Editor when creating new resource files

## 2.0.4

- Fixed a mouse wheel bug
- Added a shortcut to output an image bank

## 2.0.3

- Increased audio clock rate to 120MHz
- Updated mouse cursor position when focus is lost

## 2.0.2

- Reordered declarations in the pyi file
- Music.set no longer requires specifying all channels
- Switched to Ruff for linting and formatting Python code
- Added usage of the show and flip functions to the README files
- Added Example 14

## 2.0.1

- Removed publish of the crate to the release script
- Tile coordinate type is back to u8
- Removed source code path from the binary
- Renamed Waveform and waveforms to Tone and tones
- Updated resource file format for the tones
- CDN links now use the latest Pyxel explicitly
- Sound.set_tones now accepts digits
- Added Example 14 (still under development)

## 2.0.0

- Switched to the C version of SDL2
- Added support for resizing the colors list
- Switched screen rendering to GLSL
- Updated Pyodide to version 0.24.1
- Updated Emscripten to version 3.1.45
- Updated SDL2 to 2.24.2
- Added screen_mode function to change screen rendering type
- Added a shortcut to change the screen mode with Alt(Option)+9
- Added support for the third and fourth gamepads
- Added dither function to set dithering type
- Added images, tilemaps as system lists
- Marked the image and tilemap functions as deprecated functions
- Added channels, sounds, and musics as system lists
- Marked the channel, sounds, and music functions as deprecated functions
- Renamed reset_capture function to reset_screencast
- Renamed set_mouse_pos function to warp_mouse
- Renamed drop_files variable to dropped_files
- Removed is_fullscreen variable
- Removed input_keys variable
- Removed set_btn and set_btnv functions
- Integrated the image and refimg of Tilemap into imgsrc
- Marked the image and refimg of Tilemap as deprecated fields
- Renamed snds_list of Music to seqs
- Marked the snds_list of Music as a deprecated field
- Switched to a new resource format based on TOML
- Updated arguments for the load and save functions
- Added Waveform class for waveform editing
- Added waveforms as a system list
- Added a detune field to Channel
- Updated 8bit BGM generator to the latest version
