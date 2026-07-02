# Change Log

## 2.9.7

- Added a GIMP palette file for Pyxel's default colors
- Updated Pyodide to version 314.0.2
- Updated Rust to version nightly-2026-06-30
- Updated SDL2 to version 2.32.10 for Linux builds
- Fixed TMX imports with flipped tiles
- Fixed reversed slice assignment for Pyxel sequence objects
- Fixed possible channel desync when starting music playback
- Fixed escaping for generated HTML and browser file exports
- Fixed startup script cleanup after failed Pyxel app packaging
- Fixed palette loss when Image.from_image fails to load a file
- Fixed crash when playing or saving sounds with out-of-range tones
- Improved audio/MML/BGM processing and resource save performance
- Improved web export and Pyxel Editor shortcut performance
- Refined Japanese, Chinese, and web documentation wording and typography

## 2.9.6

- Updated Pyodide to version 314.0
- Updated Emscripten to version 5.0.3
- Updated SDL2 to version 2.32.10
- Fixed WASM SDL2 PIC linking and wheel README packaging
- Updated Rust to version nightly-2026-06-12
- Updated pyo3 crate to version 0.29
- Updated symphonia crate to version 0.6
- Updated sysinfo crate to version 0.39
- Trimmed encoder delay and padding from decoded PCM audio
- Fixed mistyped notes in Pyxel MML Studio sample tune A URL
- Raised the minimum Python version to 3.11
- Avoided Rc clone when refreshing voice tone state
- Rejected unresolved ties, invalid lengths, and unmatched repeats in MML
- Fixed profiler frame time on tick counter wraparound
- Optimized line, rectb, circ, and elli drawing with span fills
- Lazy-loaded below-the-fold images in the web user guide
- Cached API reference element lookups for search input
- Debounced Pyxel MML Studio URL and QR updates while typing
- Fixed old MML parser panic on zero tempo or note length
- Fixed Pyxel Editor crash when cutting a bank in tilemap mode
- Fixed sound editor speed display not updating when switching sounds
- Fixed negative sample rounding bias in voice gain processing
- Removed unused semver dependency, constants, and dead code
- Fixed flipped blt and bltm clipping when the source overhangs
- Fixed elli and ellib drawing with zero width or height
- Fixed tri fill when all three vertices share one row
- Fixed PCM sounds being skipped after note sounds in a playlist
- Fixed seek into sounds following a PCM sound in a playlist
- Fixed ghost notes when resuming PCM playback after an interrupting sound
- Fixed BGM generator hang on custom chords without tones
- Fixed old MML parser panic on trailing whitespace
- Fixed old resource load crashing on malformed palette files
- Fixed BDF font parse crash on overlong bitmap rows
- Fixed Pyxel Editor color pick offset at pixel boundaries
- Fixed Pyxel MML Studio legacy share URLs failing to load
- Fixed doubled HTML escaping in web user guide link labels
- Refined wording, translations, and data across web pages and docs

## 2.9.5

- Fixed aliasing UB when tilemap imgsrc points to the target image
- Switched image color matching to plain RGB Euclidean distance
- Removed unused string allocation in PyO3 type cast macro
- Hoisted clip checks out of dithered row fill loop
- Reduced WASM virtual gamepad input from 10 JS calls per frame to 1
- Cached WASM keyboard scancode correction scripts per scancode
- Prefetched WASM wheel and import hook in parallel with Pyodide load
- Enabled WASM SIMD128 and gated -Zbuild-std by the WASM target
- Switched shared types to Rc-based ownership and resolved sound leaks
- Fixed missing audio lock in several audio API call paths
- Fixed audio_bgm2 sample using off-palette colors
- Fixed Image.from_image accepting over 256 colors with include_colors
- Fixed Tone.sample_bits range allowing zero or shift-overflow values
- Fixed vibrato modulation skipped when MML period equals initial value
- Made Tone wavetable, sample_bits, and gain take effect mid-note
- Removed Tone.waveform and replaced Seq[T] with list[T] in type hints
- Added Tone.sample_bits and Channel.detune docs and refined translations

## 2.9.4

- Fixed Pyxel Editor mismapping user palettes with more than 16 colors
- Fixed Pyxel Editor color picker cursor shape across palette sizes
- Fixed missing id attributes on web pages
- Added cfg(pyxel_core) gates to audio save APIs
- Reused waveform buffer when updating tone wavetable
- Gated reset_statics and pid_exists by target OS
- Replaced gen_bgm preset clamp with a bounds assertion
- Added explicit error for BDF fonts wider than 32 pixels
- Added Python 3.14 to PyPI classifiers

## 2.9.3

- Added a JSON API to the BGM generator for Pyxel Composer integration
- Reorganized BGM generator internals and added determinism snapshot test
- Refactored editor widgets and cleaned up state handling
- Simplified Rust binding error handling
- Refined web pages, share links, and translation terminology

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
- Fixed duplicate startup script entry in packaged Pyxel apps
- Fixed app2exe output colliding with the Pyxel app source directory
- Renamed get_pixel/tile/value accessor methods to pixel/tile/value
- Added custom chord progression support to bgm_generator
- Added Pyxel Web Launcher to the showcase
- Added shortcut keys for editors
- Optimized rendering, audio, and parsing performance
- Preserved MML input case and whitespace
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
- Pinned WASM CDN imports to @main branch

## 2.8.9

- Removed unnecessary allocations in rendering and audio
- Unified editor undo/redo and input handling patterns
- Optimized drawing hot paths
- Optimized audio command processing
- Fixed memory leak in MML playback on channels
- Fixed tilemap editor selection
- Fixed tilemap viewer rendering
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

- Refactored Canvas blit paths and palette handling
- Constrained the chord note range in the BGM generator
- Renamed web i18n variables for clarity
- Fixed WASM key sticking by switching to a scancode correction map

## 2.8.5

- Optimized blt and bltm rendering with fast paths
- Optimized tilemap, text, and perspective rendering
- Optimized screen and palette texture uploads
- Increased the maximum color palette size from 255 to 256
- Optimized GIF screencast saving with buffer reuse
- Enabled Python atexit handlers on program termination
- Enabled audio playback in headless mode
- Fixed WASM keyboard keys sticking on rapid input
- Migrated User Examples from GitHub wiki to a dedicated gh-pages site

## 2.8.4

- Added the multilingual user guide
- Simplified the README
- Added shared CSS for WASM document pages
- Fixed translation inconsistencies across WASM pages
- Fixed crash in sound editor when playback reaches end of notes
- Extracted shared language detection into pyxel-pages.js
- Replaced Tailwind CDN with local CLI build for web pages
- Moved web pages from wasm/ to web/ with redirect support
- Added web usage guide page with multilingual support
- Fixed arrow keys not working in Safari on web
- Added auto-generated markdown docs from web pages
- Added auto-generated MML Commands documentation page
- Added resource file format documentation
- Opened showcase links in new tabs

## 2.8.3

- Added URL loading support to Pyxel Code Maker
- Changed Pyxel Code Maker to load default project from zip file
- Moved WASM-only images from docs/images to wasm/images
- Added docstrings to type hints from API reference
- Added multilingual editor manual with 12 language support
- Unified default width of WASM tool pages

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
- Fixed deprecated warning in voxatron.pyxapp

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

## 2.7.1

- Redesigned Pyxel Examples page as Pyxel Showcase
- Improved web tools with i18n support and other enhancements
- Added API Reference page for Pyxel Web
- Fixed type hints in pyi for Seq, Tilemap, Sound, and Music
- Removed keyword-only separators from pyi and PyO3 bindings

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
- Enabled palette file download for new files in Pyxel Editor
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

- Added two Pyxel apps by Adam for Pyxel Web Launcher
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
- Added Example 17 for Pyxel Web Launcher and the reset function
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
