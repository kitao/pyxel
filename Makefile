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
#	- libsdl2-dev 2.0.10+
#
#	[WASM]
#	- Emscripten 3.1.45 (does not work with 3.1.46+)
#
# Advance preparation:
#	rustup install nightly
#	git clone --depth=1 https://github.com/kitao/pyxel
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
WASM_DIR = $(ROOT_DIR)/wasm
WASM_ENV = RUSTUP_TOOLCHAIN=nightly
WASM_TARGET = wasm32-unknown-emscripten
CLIPPY_OPTS = -q --all-targets --all-features -- --no-deps

ifeq ($(TARGET),)
ENSURE_TARGET =
BUILD_OPTS = --release
else
ENSURE_TARGET = rustup target add $(TARGET)
BUILD_OPTS = --release --target $(TARGET)
endif

.PHONY: \
	all clean distclean lint update format build install test \
	clean-wasm build-wasm fetch-remote-wasm start-test-server test-wasm test-remote-wasm

all: build

clean:
	@cd $(CRATES_DIR)/pyxel-platform; cargo clean $(BUILD_OPTS)
	@cd $(CRATES_DIR)/pyxel-engine; cargo clean $(BUILD_OPTS)
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo clean $(BUILD_OPTS)

distclean:
	@rm -rf $(DIST_DIR)
	@rm -rf $(CRATES_DIR)/pyxel-platform/target
	@rm -rf $(CRATES_DIR)/pyxel-engine/target
	@rm -rf $(CRATES_DIR)/pyxel-wrapper/target

lint:
	@cd $(CRATES_DIR)/pyxel-platform; cargo +nightly clippy $(CLIPPY_OPTS)
	@cd $(CRATES_DIR)/pyxel-platform; cargo +nightly clippy --target $(WASM_TARGET) $(CLIPPY_OPTS)
	@cd $(CRATES_DIR)/pyxel-engine; cargo +nightly clippy $(CLIPPY_OPTS)
	@cd $(CRATES_DIR)/pyxel-engine; cargo +nightly clippy --target $(WASM_TARGET) $(CLIPPY_OPTS)
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo +nightly clippy $(CLIPPY_OPTS)
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo +nightly clippy --target $(WASM_TARGET) $(CLIPPY_OPTS)
	@flake8 $(SCRIPTS_DIR) $(PYXEL_DIR)

update:
	@cd $(CRATES_DIR)/pyxel-platform; cargo update
	@cd $(CRATES_DIR)/pyxel-engine; cargo update
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo update
	@cd $(CRATES_DIR)/pyxel-platform; cargo outdated --root-deps-only
	@cd $(CRATES_DIR)/pyxel-engine; cargo outdated --root-deps-only
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo outdated --root-deps-only

format:
	@cd $(CRATES_DIR)/pyxel-platform; cargo +nightly fmt -- --emit=files
	@cd $(CRATES_DIR)/pyxel-engine; cargo +nightly fmt -- --emit=files
	@cd $(CRATES_DIR)/pyxel-wrapper; cargo +nightly fmt -- --emit=files
	@isort $(ROOT_DIR)
	@black $(ROOT_DIR)

build: format
	@$(ENSURE_TARGET)
	@$(SCRIPTS_DIR)/make_abspath_readme
	@maturin build -o $(DIST_DIR) $(BUILD_OPTS) --manylinux 2014 --skip-auditwheel

install: build
	@pip3 install --force-reinstall `ls -rt $(DIST_DIR)/*.whl | tail -n 1`

test: install
	@cd $(CRATES_DIR)/pyxel-engine; cargo test $(BUILD_OPTS)
	@python3 -m unittest discover $(CRATES_DIR)/pyxel-wrapper/tests
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
	@pyxel run $(EXAMPLES_DIR)/13_bitmap_font.py
	@pyxel run $(EXAMPLES_DIR)/14_synthesizer.py
	@pyxel run $(EXAMPLES_DIR)/99_flip_animation.py
	@pyxel play $(EXAMPLES_DIR)/30SecondsOfDaylight.pyxapp
	@pyxel play $(EXAMPLES_DIR)/megaball.pyxapp
	@pyxel play $(EXAMPLES_DIR)/8bit-bgm-gen.pyxapp
	@pyxel edit $(EXAMPLES_DIR)/assets/sample.pyxres
	@rm -rf testapp testapp.pyxapp
	@mkdir -p testapp/assets
	@cp $(EXAMPLES_DIR)/10_platformer.py testapp
	@cp $(EXAMPLES_DIR)/assets/platformer.pyxres testapp/assets
	@pyxel package testapp testapp/10_platformer.py
	@pyxel play testapp.pyxapp
	@rm -rf testapp testapp.pyxapp
	@pyxel watch $(EXAMPLES_DIR) $(EXAMPLES_DIR)/01_hello_pyxel.py

clean-wasm:
	@$(WASM_ENV) make clean TARGET=$(WASM_TARGET)

build-wasm:
	@embuilder build sdl2 --pic
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@$(WASM_ENV) make build TARGET=$(WASM_TARGET)
	@$(SCRIPTS_DIR)/install_wasm_wheel

fetch-remote-wasm:
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@$(SCRIPTS_DIR)/download_wasm_wheel
	@$(SCRIPTS_DIR)/install_wasm_wheel

start-test-server:
	$(SCRIPTS_DIR)/switch_html_scripts local
	@bash -c " \
		trap '$(SCRIPTS_DIR)/switch_html_scripts cdn' INT TERM; \
		$(SCRIPTS_DIR)/start_test_server \
	"

test-wasm: build-wasm start-test-server

test-remote-wasm: fetch-remote-wasm start-test-server
