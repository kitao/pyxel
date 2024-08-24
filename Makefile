#
# [How to build]
#
# Required Tools:
#	- git
#	- make
#	- cmake
#	- rustup
#	- python 3.8+
#
#	[Windows]
#	- Git Bash
#
#	[Linux]
#	- python3-pip
#	- python3-venv
#	- libsdl2-dev 2.28.4
#
#	[Web]
#	- Emscripten 3.1.61 (does not work with 3.1.62+)
#
# Advance Preparation:
#	rustup install nightly
#	git clone --depth=1 https://github.com/kitao/pyxel
#	cd pyxel
#	(Create and activate a venv if you prefer)
#	pip3 install -r python/requirements.txt
#
# Build for Current Environment:
#	make clean build
#	(Generates Python wheel in dist/ directory)
#
# Build for Specified Target:
#	make clean build TARGET=target_triple
#
# Build for Specified Target:
#	make clean build TARGET=target_triple
#
# Build and Install in Current Python:
#	make clean install
#
# Build, Install, and Test in Current Python:
#	make clean test
#
# Build for Web:
#	make clean-wasm build-wasm
#
# Test for Web:
#	make clean-wasm test-wasm
#	(Open localhost:8000/wasm/ in a web browser)
#

ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
RUST_DIR = $(ROOT_DIR)/rust
PYTHON_DIR = $(ROOT_DIR)/python
EXAMPLES_DIR = $(PYTHON_DIR)/pyxel/examples
SCRIPTS_DIR = $(ROOT_DIR)/scripts
WASM_DIR = $(ROOT_DIR)/wasm
WASM_ENV = RUSTUP_TOOLCHAIN=nightly
WASM_TARGET = wasm32-unknown-emscripten

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
	@cd $(RUST_DIR); cargo clean $(BUILD_OPTS)

distclean:
	@rm -rf $(DIST_DIR)
	@rm -rf $(RUST_DIR)/target

lint:
	@cd $(RUST_DIR); cargo +nightly clippy -q --all-targets --all-features -- --no-deps
	@cd $(RUST_DIR); cargo +nightly clippy --target $(WASM_TARGET) -q --all-targets --all-features -- --no-deps
	@ruff check $(ROOT_DIR)

update:
	@rustup -q update
	@cd $(RUST_DIR); cargo -q update
	@cd $(RUST_DIR); cargo -q outdated --root-deps-only
	@pip3 -q install -U -r $(PYTHON_DIR)/requirements.txt

format:
	@cd $(RUST_DIR); cargo +nightly fmt -- --emit=files
	@ruff format $(ROOT_DIR)

build: format
	@$(ENSURE_TARGET)
	@$(SCRIPTS_DIR)/make_abspath_readme
	@cd $(PYTHON_DIR); maturin build -o ../$(DIST_DIR) $(BUILD_OPTS) --manylinux 2014 --skip-auditwheel

install: build
	@pip3 install --force-reinstall `ls -rt $(DIST_DIR)/*.whl | tail -n 1`

test: install
	#@cd $(RUST_DIR); cargo test $(BUILD_OPTS)
	@python3 -m unittest discover $(RUST_DIR)/pyxel-wrapper/tests
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
	@pyxel run $(EXAMPLES_DIR)/15_tiled_map_file.py
	@pyxel run $(EXAMPLES_DIR)/16_transform.py
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
	@bash -c "trap '$(SCRIPTS_DIR)/switch_html_scripts cdn' INT TERM; $(SCRIPTS_DIR)/start_test_server"

test-wasm: build-wasm start-test-server

test-remote-wasm: fetch-remote-wasm start-test-server
