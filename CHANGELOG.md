# Change Log

## 1.0.1
- Simplified comparisons
- Removed a deprecated function
- Applied static decorator to functions do not use self
- Fixed to keep the previous frame when cls is not called
- Changed frame_count to start from 0
- Fixed the gamepad constants

## 1.0.0
- Added the supplement of installation method to the manuals
- Added the type hints for Python

## 0.9.10
- Added the way to import images on Pixel Editor to the manuals
- Fixed the type hints
- Added GLFW dll for Windows

## 0.9.9
- Added the type hints for Python
- Added the description of run_with_profiler function to the manuals

## 0.9.8
- Enabled to run the Pyxel Editor without filename

## 0.9.7
- Confirmed operation on Debian and Fedora
- Updated the instruction of installation on Linux

## 0.9.6
- Updated the instruction of installation on Linux
- Fixed a warning in setup.py

## 0.9.5
- Added issue templates
- Added the description of issue templates in the manuals
- Fixed the clipping bug of drawing primitives

## 0.9.4
- Fixed the crush bug when entering the fullscreen mode
- Updated the description for Linux in the manuals
- Reverted to check the version number of glfw strictly
- Increased the sound buffer size
- Fixed the range of the sound picker in the Sound Editor

## 0.9.3
- Enabled to open a resource file by drop in the Pyxel Editor
- Renamed the constants for the mouse buttons
- Added gamepad support
- Changed gamepad available for the example #2
- Fixed the crash bug when the window is minimized
- Modified the code of the example #6
- Added the refimg property to the Tilemap class
- Removed the img argument from the bltm command
- Updated the screenshot of the example #3

## 0.9.2
- Enabled to import png by drop in the Image Editor
- Fixed the crash bug caused by unsupported keys
- Enabled to play the piano with mouse in the Sound Editor
- Enabled to repeat undo/redo shortcuts

## 0.9.1
- Fixed the color pick bug of the Image Editor
- Changed the focus UI of the Image Editor

## 0.9.0
- Fixed the bug where the Pyxel Editor cursor malfunctions
- Added new API descriptions to the example #3
- Updated the screenshot of the example #3
- Change not to add unnecessary undo history of the editors
- Added the setting files for Pipenv
- Modified the cursor design of the Image and Tilemap Editors
- Enabled to change the focus size of the Tilemap Editor
- Added the link to the subreddit in the manuals
- Changed to exports all constants for keys before init is called
- Added the contribution section to the manuals

## 0.8.9
- Fixed the tilemap to allow the tiles of #256 or higher
- Updated the screenshots of the Pyxel Editor
- Fixed the cursor movement of the sound and music editors
- Changed the caption of the example #2
- Renamed the example #6
- Fixed the error when saving long animated GIF

## 0.8.8
- Added the .pyxel file to the install example script

## 0.8.7
- Fixed the piano keyboard bug when only enter was pressed
- Fixed the piano keyboard hilights correctly
- Changed the default sound volume to 7
- Changed the default sound speed to 30
- Fixed the sound button bug of the sound editor

## 0.8.6
- Changed the operation method of the sound editor
- Changed to allow sound of length 0
- Implemented the music editor
- Changed the example #2 to use resource file

## 0.8.5
- Implemented the undo function of the sound editor
- Changed the click tolerance time
- Removed the length limitation of the sound
- Added the music and playm commands
- Changed the example #2 to use the music and playm commands

## 0.8.4
- Changed to follow the mouse position outside the window
- Changed to draw the self mouse cursor
- Added the mouse command
- Renamed arguments of the blt and bltm commands
- Reduced the size of PNG and animated GIF
- Changed the max length of the sound to 48
- Added the system option to the sound command
- Refined the help message of the Pyxel Editor
- Added the ToggleButton and ImageToggleButton
- Implemented the sound editor except the undo function

## 0.8.3
- Fixed the right click bug in the Tilemap Editor
- Fixed the key callback bug
- Added the get method to the Image class
- Changed the set method of the Image class to accept a number as data
- Added the get and set methods to the Tilemap class
- Added the bltm test to the example #3
- Updated the descriptions of the READMEs

## 0.8.2
- Fixed the set, load, and copy methods of the image class
- Fixed the starting directory of the save and load commands
- Modified the usage of the Pyxel Editor

## 0.8.1
- Added the run_with_profiler command
- Added the Tilemap class
- Added the bltm command
- Implemented the tilemap editor
- Added the standard widgets
- Changed the usage of the Pyxel Editor
- Added the help messages to the Pyxel Editor
- Added the description of the Pyxel Editor to the READMEs

## 0.8.0
- Changed the formatter from yapf to black
- Added the UI module
- Added the save and load commands
- Added the Image Editor as a part of the Pyxel Editor
- Removed the dirname option of the save and load commands

## 0.7.12
- Added new key definitions which integrates keys on both sides
- Changed the description of the project
- Changed the max scren size to 255
- Fixed the key hold time of the btnp command
- Fixed to work the btnp command correctly in the slow frame rate condtion
- Changed the screen size of the example #5 and #6
- Updated the screenshot of the example #5

## 0.7.11
- Changed the Japanese link name in the READMEs
- Implemented the widget class for the resource editor
- Added the example #6 by ttrkaya
- Renamed the constant variable for the max screen size
- Changed to set the defualt scale automatically

## 0.7.10
- Added screen size error
- Added the link to the Pyxel wiki to the READMEs
- Moved the README in Portuguese to the Pyxel wiki

## 0.7.9
- Changed the color of the system font texture to white
- Fixed a typo in the README.md
- Renamed the modules to use snake case
- Added glfw version check

## 0.7.8
- Added the system option to the image command
- Fixed the color bug of the shader for some environments
- Added the dirname option to the load method of the image class
- Updated the description of the init command of the READMEs

## 0.7.7
- Added the description of the screen size limitation to the READMEs
- Added the Fedora Linux installation to the READMEs
- Added another fallback to get the desktop folder name
- Changed the number of the image banks to 3
- Added some image assets for Pyxel Editor (WIP)

## 0.7.6
- Specify the version of GLFW in the READMEs
- Limited the window size to 256 because of OpenGL Point Sprite limitation
- Fixed the element border lacks bug
- Added the example #5

## 0.7.5
- Fixed typos in the READMEs
- Updated the Arch Linux installation in the READMEs
- Updated the Debian Linux installation in the READMEs
- Updated the way to capture screen on Linux
- Fixed a shader compile error occurs in some environment

## 0.7.4
- Fixed to run without an audio card
- Refactored import order of all files with isort
- Fixed the way to capture screen on Linux

## 0.7.3
- Fixed the btnr command
- Fixed a typo in the README.md
- Added the title logo to the READMEs
- Added the Portuguese version of the README.md

## 0.7.2
- Changed not to include the screenshots in the PyPI package
- Removed unnecessary semicolons in the shader to avoid compile errors
- Changed the project description for PyPI
- Added the description of installation on Linux to the READMEs
- Refactored the way to make a captured image and animation
- Updated the screenshots of the example #3 and #4

## 0.7.1
- Modified the bgm of the example #2
- Renamed the argument 'no' of the image-related methods to 'img'
- Renamed the argument 'no' of the sound-related methods to 'snd'
- Fixed to include the assets and screenshots in the PyPI package

## 0.7.0
- Modified the example #1 to use the App class
- Renamed and modified the example #2
- Remove the logo command and added the logo image
- Improved the performance of the text command
- Updated the README.md and README.ja.md
- Replaced the example #2
- Removed the resize method of the Image class
- Changed the size of the Image to 256x256
- Fixed the copy method of the Image class

## 0.6.0
- Changed the properties of the Sound class to public
- Added offset arguments to the Image load method
- Added the copy method to the Image class
- Renamed arguments of the image and sound command
- Added the window icon
- Added the logo command
- Added the resize method to the Image class
- Refined the example #1-4

## 0.5.0
- Added the version number constant
- Renamed the examples copy script to install_pyxel_examples
- Removed unnecessary scripts
- Separated the constant definitions
- Added the image command and renamed related APIs
- Added the sound command and renamed related APIs

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
- Changed the play command to enable to play a sound list

## 0.2.0
- Added the audio playback function
- Removed the set method of the Image class

## 0.1.0
- First alpha release
