# Change Log

## 3.0.0

- Added the Pyxel Cube software-rendered 3D extension module
- Removed old MML syntax and pre-2.0 resource loading
- Updated Pyodide to version 314.0.6
- Added an app2exe install extra for PyInstaller
- Improved Python type hints and catchable API errors
- Fixed blit clipping, self-copying, flood fills, and tilemap collisions
- Fixed scaled or rotated bltm drawing tiles outside the image as color 0
- Fixed GIF capture colors and failures on large captures
- Fixed screenshot and image saving with colors beyond the palette
- Fixed key repeat, modifier states, and mouse positioning
- Fixed frame timing during long-running sessions
- Fixed sound playback, sequence data loss, and BGM generation hangs
- Fixed same-pitch MML ties ignoring the Q gate time
- Fixed resource saving, version parsing, and TMX imports
- Fixed sequence access failures and deadlocks during Python garbage collection
- Fixed unwanted data changes, import errors, and save failures in Pyxel Editor
- Fixed drawing and wavetable editing in Pyxel Editor and examples
- Fixed tile range selection by dragging in Pyxel Editor
- Kept the title on the save file after dropping a resource in Pyxel Editor
- Fixed app packaging paths, overwrites, and directory symlinks
- Fixed app2exe imports and concurrent builds
- Fixed play and watch process cleanup
- Fixed web startup, imports, and virtual gamepad resizing
- Fixed project loading, saving, downloads, and share URLs in web tools
- Improved web layouts, accessibility, and API reference search
- Improved blit and primitive drawing performance and reduced audio allocations
- Fixed OpenGL cleanup on application shutdown
- Used the desktop OpenGL shader version on macOS
- Improved wheel builds and SDL2 download verification
- Improved documentation and translations

## 2.9.9

- Updated Rust to nightly-2026-08-12 and Pyodide to 314.0.4
- Updated blip_buf to 0.2 and pyo3 to 0.29.2
- Added browser downloads for sound and music saves on Pyxel Web
- Fixed browser export filenames and empty screencast saves
- Fixed Image.from_image palette corruption with more than 256 colors
- Fixed app2exe imports of package submodules
- Fixed Pyxel Editor resource drops and number picker input
- Fixed Web Launcher and Code Maker share URLs pinning to a commit SHA

## 2.9.8

- Updated Rust to nightly-2026-07-14 and glow to 0.18
- Fixed crashes and partial updates from malformed resource and image data
- Fixed API calls during frame callbacks and window state preservation
- Fixed Web asset loading, imports, and memory retained across resets
- Fixed MML timing, note transitions, and resuming mixed PCM/note playback
- Fixed audio hangs from extreme tempos, pitches, and zero-duration repeats
- Fixed playing and saving sounds with empty tone banks
- Rejected invalid playback start times and zero-sample save durations
- Rejected invalid sound speeds, music indexes, and tone sample widths
- Fixed FFmpeg failures being reported as successful saves
- Improved rendering and audio performance

## 2.9.7

- Added a GIMP palette file for Pyxel's default colors
- Updated Pyodide to 314.0.2 and Rust to nightly-2026-07-05
- Updated SDL2 to 2.32.10 for Linux builds
- Fixed TMX imports with flipped tiles
- Fixed reversed slice assignment for Pyxel sequence objects
- Fixed music channel synchronization and audio after macOS app restarts
- Fixed playing or saving sounds with out-of-range tones
- Fixed escaping in generated HTML and browser file exports
- Fixed app packaging cleanup and palette loss on failed image imports
- Improved audio, resource saving, Web export, and editor performance
- Improved documentation and translations

## 2.9.6

- Raised the minimum Python version to 3.11
- Updated Pyodide to 314.0, Emscripten to 5.0.3, and SDL2 to 2.32.10
- Updated Rust to nightly-2026-06-12
- Updated pyo3 to 0.29, symphonia to 0.6, and sysinfo to 0.39
- Fixed WASM linking and wheel README packaging
- Trimmed encoder delay and padding from decoded PCM audio
- Rejected unresolved ties, invalid lengths, and unmatched repeats in MML
- Fixed primitive drawing and flipped blit clipping
- Fixed PCM playlist playback, seeking, and resuming after interruptions
- Fixed BGM generation hanging on empty custom chords
- Fixed crashes on malformed MML, resource palettes, and BDF fonts
- Fixed Pyxel Editor bank cutting, speed display, and color picking
- Fixed Pyxel MML Studio sample and legacy share URLs
- Improved drawing performance and Web page responsiveness
- Improved audio gain accuracy and profiler timing
- Improved documentation and translations

## 2.9.5

- Fixed self-referencing tilemap rendering
- Switched image color matching to plain RGB Euclidean distance
- Fixed Image.from_image accepting over 256 colors with include_colors
- Fixed Tone.sample_bits range allowing zero or shift-overflow values
- Fixed vibrato modulation skipped when MML period equals initial value
- Made Tone wavetable, sample_bits, and gain take effect mid-note
- Fixed sound memory leaks and audio synchronization
- Enabled WASM SIMD128 and improved Web startup and input performance
- Improved drawing performance
- Removed Tone.waveform and replaced Seq[T] with list[T] in type hints
- Added Tone.sample_bits and Channel.detune docs and refined translations

## 2.9.4

- Fixed Pyxel Editor palettes and color picking with more than 16 colors
- Rejected invalid gen_bgm presets
- Added explicit error for BDF fonts wider than 32 pixels
- Added Python 3.14 to PyPI classifiers

## 2.9.3

- Improved Web pages, share links, and translations

## 2.9.2

- Updated the bundled Pyxel wheel to 2.9.2

## 2.9.1

- Added line numbers to palette file parsing errors
- Fixed palette loader failing on whitespace-only lines
- Updated Pyxel MML Studio to use shorter share URLs
- Fixed Pyxel Web Launcher to load the latest version of each user app
- Enabled thin LTO and inlining hints in release builds
- Prevented zip path traversal in the play and app2exe commands
- Fixed relative path handling in the package command
- Fixed app2exe output colliding with the Pyxel app source directory
- Renamed get_pixel/tile/value accessor methods to pixel/tile/value
- Added custom chord progression support to bgm_generator
- Added Pyxel Web Launcher to the showcase
- Optimized rendering, audio, and parsing performance
- Refined documentation terminology and translations

## 2.9.0

- Redesigned the gen_bgm function to share code with Pyxel Composer
- Added the transp argument back to the gen_bgm function
- Made the transp, instr, and seed of the gen_bgm function required
- Added Cargo.lock to version control for reproducible builds
- Added the resize function to change the screen size at runtime
- Fixed wrong screencast last-frame delay on frame drops
- Raised minimum Python version to 3.10

## 2.8.10

- Fixed WASM public API functions lost by const refactor
- Renamed user guide 'Tools' section to 'Examples & Tools'

## 2.8.9

- Improved rendering and audio performance
- Fixed memory leaks in MML playback
- Fixed tilemap editor selection and rendering
- Removed undefined constants from type hints and API reference
- Added frame pipeline and input injection to headless mode

## 2.8.8

- Added GitHub issue and discussion templates
- Fixed MML audio engine edge cases
- Fixed editor input and copy/paste issues
- Fixed temporary image, tilemap, and sound memory leaks
- Fixed doc generator stripping HTML tags inside inline code
- Changed the tilemap tile coordinate type from u8 to u16
- Fixed mouse coordinate handling on startup

## 2.8.7

- Updated script-test.html showcase example
- Fixed headless mode to run the same frame loop as normal mode
- Added SIGINT handling to allow Ctrl+C during event loop
- Added filename option to screenshot and screencast
- Removed the 'packages' option from Pyxel Web Launcher

## 2.8.6

- Fixed Web keyboard keys sticking

## 2.8.5

- Improved drawing, display updates, and GIF capture performance
- Increased the maximum color palette size from 255 to 256
- Enabled Python atexit handlers on program termination
- Enabled audio playback in headless mode
- Fixed Web keyboard keys sticking on rapid input
- Migrated User Examples from GitHub wiki to a dedicated gh-pages site

## 2.8.4

- Added multilingual guides and simplified the README
- Moved Web pages to web/ with redirects from their previous URLs
- Replaced Tailwind CDN with a local build
- Fixed arrow keys not working in Safari
- Fixed sound editor crashes when playback reaches the end of notes

## 2.8.3

- Added URL loading support to Pyxel Code Maker
- Changed Pyxel Code Maker to load its initial project from a ZIP file
- Added docstrings to type hints from the API reference
- Added the editor manual in 12 languages

## 2.8.2

- Prioritized system SDL2 over bundled SDL2 on Linux
- Added external file drop support for web-based editor
- Fixed crash when dropping non-image files on the image editor
- Fixed crash when dropping non-TMX files on the tilemap editor

## 2.8.1

- Removed URL loading from Pyxel Code Maker
- Added system SDL2 fallback for non-X11/Wayland Linux environments

## 2.8.0

- Reverted default window icon padding
- Added project sharing via Gist, GitHub, and URL to Pyxel Code Maker
- Changed Pyxel Web Launcher URL format from dot to slash separators
- Added drag-and-drop for .py and .pyxres files in Pyxel Code Maker
- Fixed deprecated warning in vortexion.pyxapp

## 2.7.12

- Added channel count info to the gen_bgm instr descriptions
- Added preset mood descriptions to the gen_bgm API reference
- Fixed quit causing fatal error on web

## 2.7.11

- Added headless guards to platform facade layer

## 2.7.10

- Added the headless argument to the init function
- Added padding to default window icon for better OS integration

## 2.7.9

- Aligned the 3D coordinate system so rot=(0,0,0) matches 2D screen axes
- Renamed the cam parameter to pos in 3D drawing functions
- Fixed reversed FOV controls (T/G keys) in the perspective example

## 2.7.8

- Unified README installation instructions across platforms
- Simplified venv setup with --upgrade-deps
- Fixed inconsistent naming and return types in Rust and Python
- Added console.error output for WASM runtime errors
- Added Example 19 for perspective rendering
- Added the blt3d and bltm3d functions for pseudo-3D perspective rendering
- Updated glow crate to version 0.17

## 2.7.7

- Added Wayland native driver preference
- Migrated Linux CI to manylinux_2_28 (dropped i686 support)
- Fixed mouse input on Linux with cross-compiled SDL2

## 2.7.6

- Fixed mouse not working on Wayland with XWayland fallback
- Consolidated manylinux options in Makefile

## 2.7.5

- Fixed SDL2 mouse handling on Wayland and outside the window

## 2.7.4

- Fixed module import failure after directory change
- Added VS Code extension info to the README files
- Fixed mouse not working on Wayland and in virtual machines

## 2.7.3

- Improved error handling and unified error messages
- Rewrote examples 06 and 07 for consistency
- Added VS Code extension
- Added MCP server for AI-assisted development
- Added KMSDRM video driver for embedded Linux devices

## 2.7.2

- Added manual pages for Pyxel Code Maker and Pyxel MML Studio
- Removed the version number from the pyxel command in the README files
- Revamped Pyxel Web documentation and the FAQ for clarity
- Removed keyword-only separators from pyi and PyO3 bindings

## 2.7.1

- Redesigned Pyxel Examples page as Pyxel Showcase
- Improved web tools with i18n support and other enhancements
- Added API Reference page for Pyxel Web
- Fixed type hints in pyi for Seq, Tilemap, Sound, and Music

## 2.7.0

- Removed mutexes for resource types to improve performance
- Renamed internal Python module to pyxel_binding
- Optimized drawing performance with bulk fills and reduced overhead
- Renamed the pyxel-core public API to idiomatic Rust names
- Refactored imports and cleanup across Rust, Python, and JS

## 2.6.9

- Restructured project layout and reorganized Rust crates
- Removed crates.io publishing from release workflow
- Added pre-release versioning support for PyPI
- Added Python list-like operations to sequence types
- Fixed Linux SDL2 build to include X11/Wayland/ALSA/PulseAudio drivers

## 2.6.8

- Bundled SDL2 in Linux wheels for easier installation
- Renamed the SDL2 feature flags to sdl2_system and sdl2_bundle
- Updated Linux CI SDL2 version to 2.32.0

## 2.6.7

- Refined the Makefile for safer WASM builds
- Renamed and simplified tools scripts
- Updated Pyxel thanks image
- Fixed keyboard input for non-US layouts on the web version
- Removed the version update check from the pyxel command

## 2.6.6

- Reduced synth load with fixed-point amplitude mixing
- Reduced PCM load with fixed-point mixing
- Fixed the gen_bgm function parity and seed determinism
- Removed the transp argument from the gen_bgm function
- Added BGM generation feature to Example 15
- Updated toml crate to version 1.0
- Updated zip crate to version 8.1

## 2.6.5

- Removed unnecessary files to reduce the wheel size
- Added the gen_bgm function for automatic BGM generation and playback
- Added the clamp function
- Added the collide method to Tilemap
- Updated Examples 10 and 15 to use collide
- Updated the sgn function to preserve input types in Python
- Updated rand crate to version 0.10
- Updated rand_xoshiro crate to version 0.8

## 2.6.4

- Removed unnecessary files to reduce the wheel size

## 2.6.3

- Adjusted initialization order for the web version
- Normalized HTML doctypes to lowercase
- Added the pcm method to Sound for audio playback
- Added Example 18 for audio playback
- Renamed incl_colors to include_colors in Image

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

- Updated web version to display all errors
- Improved error handling for the web version
- Updated gif crate to version 0.14
- Updated zip crate to version 7.0
- Updated the quit function behavior in the web version
- Downgraded the image crate to version 0.24

## 2.5.10

- Updated Pyodide to version 0.29.0
- Updated pyo3 crate to version 0.27
- Added Pyxel Code Maker web page
- Added Pyxel Code Maker zip file support to the play command

## 2.5.9

- Added the load_pal and save_pal functions
- Enabled palette file loading for new files in Pyxel Editor
- Enabled automatic color picker size adjustment in Pyxel Editor
- Updated Pyodide to version 0.28.3
- Updated Emscripten to version 4.0.9
- Updated SDL2 to version 2.32.0
- Updated web pages to refer to the main branch
- Updated zip crate to version 6.0

## 2.5.8

- Set the desktop OpenGL internal format to GL_R8
- Fixed Tilemap.data_ptr to expose full map data
- Updated build environment version for Mac to macOS 15

## 2.5.7

- Specified Tailwind CSS version 3.4.17 for web pages
- Added links to web tools and examples for Pyxel in the README files
- Updated URL on reload in Pyxel MML Studio
- Improved usability of Pyxel MML Studio
- Enabled parent HTML window to control the initial input wait

## 2.5.6

- Updated Pyxel MML Studio to use compressed URLs
- Improved usability of Pyxel MML Studio
- Updated design of the web pages

## 2.5.5

- Updated exe packaging for the reset function with PyInstaller
- Fixed touch device detection for Firefox in the web version
- Reworked the reset function and the play command behavior
- Removed extra directories after the app2exe command
- Improved automatic file download for the web version
- Adjusted error output display size in the web version
- Updated tone selection to use 0 for non-existent tone numbers
- Separated the web MML commands into the Pyxel MML Studio page

## 2.5.4

- Added two Pyxel apps by Adam for the app launcher
- Fixed Example 17 Python command execution issue
- Fixed the reset function issue when called inside pyxapp
- Updated design of the web pages
- Updated zip crate to version 5.0

## 2.5.3

- Updated HTML pages to use the latest Pyxel from CDN
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

- Updated the reset function to preserve environment variables
- Fixed cargo publish error by adding features sdl2
- Added line break support for custom font rendering
- Fixed app2exe issue with white spaces

## 2.5.0

- Refactored the platform abstraction layer
- Reduced error output in the web version
- Added the reset function
- Added automatic use of old_mml when '~' is used
- Fixed delayed sound playback on Android browsers
