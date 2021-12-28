#
# [How to build]
#
# Prerequisites for Windows:
#   - Build Tools for Visual Studio 2019
#   - Cygwin64 with make and zip added
#   - Inno Setup
#
# Prerequisites for Mac:
#   - Xcode
#   - `sudo xcode-select --switch /Applications/Xcode.app`
#
# Prerequisites for Linux:
#   - SDL2 package (libsdl2-dev for Ubuntu)
#
# Common prerequisites:
#   - cmake
#   - rust and cargo
#   - python3 and pip3 (or python and pip for Windows)
#   - pyoxidizer
#
# Format and lint code:
#   make format
#
# Build:
#   make clean all
#
# Build and test:
#   make clean test
#
# Install Python pakcage after build:
#   pip3 install .
#   (Prefix `sudo` for Linux)
#
# Make Python wheel after build on all platforms:
#   make wheel
#
# Make Pyxel distributions using Python wheel:
#   make dist
#

FORWARD_DIR = lib

.PHONY: forward $(MAKECMDGOALS)

forward:
	@$(MAKE) -C $(FORWARD_DIR) $(MAKECMDGOALS)

$(MAKECMDGOALS): forward
