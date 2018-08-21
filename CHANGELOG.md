# Change Log

## 0.7.11
- Changed the Japanese link name in the READMEs
- Added the script to format python code
- Implemented the widget class for the resource editor
- Added the example #6 by ttrkaya

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
