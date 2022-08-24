#
# [How to build]
#
# Required tools:
#	- git
#	- make
#	- cmake
#	- rustup
#	- python 3.7+
#
#	[Windows]
#	- Git Bash
#
#	[Linux]
#	- python3-pip
#	- python3-venv
#
#	[WASM]
#	- Emscripten 3.1.14
#
# Advance preparation:
#	rustup install nightly
#	git clone --depth 1 https://github.com/kitao/pyxel.git
#	cd pyxel
#	(Create and activate a venv if you prefer)
#	pip3 install -r requirements.txt
#
# Build the package in the dist directory
#	make clean build
#
# Build the package for the specified target:
#	make clean build TARGET=target_triple
#
# Build, install, and test the package in the current Python
#	make clean test
#
# Build the package for WASM in the dist directory
#	make clean-wasm build-wasm
#
# Test the package for WASM in localhost:8000/wasm/
#	make clean-wasm test-wasm
#

ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
PYXEL_DIR = $(ROOT_DIR)/python/pyxel
CRATES_DIR = $(ROOT_DIR)/crates
SCRIPTS_DIR = $(ROOT_DIR)/scripts
EXAMPLES_DIR = $(PYXEL_DIR)/examples
SRC_SDL2 = $(CRATES_DIR)/pyxel-wrapper/target/$(TARGET)/release/SDL2.dll
DST_SDL2 = $(PYXEL_DIR)/SDL2.dll
WASM_ENVVARS = RUSTUP_TOOLCHAIN=nightly
WASM_TARGET = wasm32-unknown-emscripten

ifeq ($(COMSPEC),)
PYTHON = python3
PIP = pip3
else # Windows
PYTHON = python
PIP = pip
endif

ifeq ($(TARGET),)
ADD_TARGET =
BUILD_OPTS = --release
else
ADD_TARGET = rustup target add $(TARGET)
BUILD_OPTS = --release --target $(TARGET)
endif

.PHONY: all clean distclean lint format build test clean-wasm build-wasm test-wasm

all: build

clean:
	@cd $(CRATES_DIR)/pyxel-engine; cargo clean $(BUILD_OPTS)
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo clean $(BUILD_OPTS)

distclean:
	@rm -rf $(CRATES_DIR)/pyxel-engine/target
	@rm -rf $(CRATES_DIR)/pyxel-wrapper/target

lint:
	@cd $(CRATES_DIR)/pyxel-engine; cargo clippy -q -- --no-deps
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo clippy -q -- --no-deps
	@flake8 $(SCRIPTS_DIR) $(PYXEL_DIR)

format:
	@cd $(CRATES_DIR)/pyxel-engine; cargo +nightly fmt -- --emit=files
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo +nightly fmt -- --emit=files
	@isort $(ROOT_DIR)
	@black $(ROOT_DIR)
	@$(SCRIPTS_DIR)/update_readme

build: format
	@$(ADD_TARGET)
	@rm -f $(DST_SDL2)
	@maturin build -o $(DIST_DIR) $(BUILD_OPTS)
	@if [ -e $(SRC_SDL2) ]; then \
		cp $(SRC_SDL2) $(DST_SDL2); \
		maturin build -o $(DIST_DIR) $(BUILD_OPTS); \
		rm $(DST_SDL2); \
	fi

test: build
	@cd $(CRATES_DIR)/pyxel-engine; cargo test $(BUILD_OPTS)
	@$(PIP) install --force-reinstall `ls -rt $(DIST_DIR)/*.whl | tail -n 1`
	@$(PYTHON) -m unittest discover $(CRATES_DIR)/pyxel-wrapper/tests

	@pyxel run $(EXAMPLES_DIR)/01_hello_pyxel.py
	@pyxel run $(EXAMPLES_DIR)/02_jump_game.py
	@pyxel run $(EXAMPLES_DIR)/03_draw_api.py
	@pyxel run $(EXAMPLES_DIR)/04_sound_api.py
	@pyxel run $(EXAMPLES_DIR)/05_color_palette.py
	@pyxel run $(EXAMPLES_DIR)/06_click_game.py
	@pyxel run $(EXAMPLES_DIR)/07_snake.py
	@pyxel run $(EXAMPLES_DIR)/08_triangle_api.py
	@pyxel run $(EXAMPLES_DIR)/09_shooter.py
	@pyxel run $(EXAMPLES_DIR)/10_platformer.py
	@pyxel run $(EXAMPLES_DIR)/11_offscreen.py
	@pyxel run $(EXAMPLES_DIR)/12_perlin_noise.py
	@pyxel play $(EXAMPLES_DIR)/30SecondsOfDaylight.pyxapp
	@pyxel play $(EXAMPLES_DIR)/megaball.pyxapp
	@pyxel edit $(EXAMPLES_DIR)/assets/sample.pyxres

	@rm -rf testapp testapp.pyxapp
	@mkdir -p testapp/assets
	@cp $(EXAMPLES_DIR)/10_platformer.py testapp
	@cp $(EXAMPLES_DIR)/assets/platformer.pyxres testapp/assets
	@pyxel package testapp 10_platformer.py
	@pyxel play testapp.pyxapp
	@rm -rf testapp testapp.pyxapp

clean-wasm:
	@$(WASM_ENVVARS) make clean TARGET=$(WASM_TARGET)

build-wasm:
	@$(WASM_ENVVARS) make build TARGET=$(WASM_TARGET)

test-wasm: build-wasm
	@cp -f $(DIST_DIR)/*-emscripten_*.whl $(ROOT_DIR)/wasm
	@$(SCRIPTS_DIR)/start_server
