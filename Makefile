#
# [How to build]
#
# Prerequisites for Windows:
#   - Build Tools for Visual Studio 2019
#   - Cygwin64 with make and zip added
#   - Inno Setup
#
# Common prerequisites:
#   - cmake
#   - rustup
#   - python3 (or python for Windows)
#   - pyoxidizer by `cargo install pyoxidizer`
#
# Format and lint code:
#   make format
#
# Debug build:
#   make clean all
#
# Debug build and test:
#   make clean test
#
# Release build:
#   make clean all RELEASE=1
#
# Release build and test:
#   make clean test RELEASE=1
#
# Install Python pakcage after release build:
#   pip3 install .
#
# Make Python wheel after release build:
#   make wheel
#
# Make Pyxel distributions using Python wheel:
#   make dist
#
# Make Pyxel installer using Windows package:
#   Build native/setup.iss with Inno Setup
#

FORWARD_DIR = native

.PHONY: forward $(MAKECMDGOALS)

forward:
	@$(MAKE) -C $(FORWARD_DIR) $(MAKECMDGOALS)

$(MAKECMDGOALS): forward
