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
#	- Emscripten 3.1.58 (the same version used by Pyodide)
#
# Advance Preparation:
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

# Project directories
ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
RUST_DIR = $(ROOT_DIR)/rust
PYTHON_DIR = $(ROOT_DIR)/python
EXAMPLES_DIR = $(PYTHON_DIR)/pyxel/examples
SCRIPTS_DIR = $(ROOT_DIR)/scripts
WASM_DIR = $(ROOT_DIR)/wasm

# Build targets
WASM_TARGET = wasm32-unknown-emscripten

# Tool options
CLIPPY_OPTS = -q --all-targets --all-features -- --no-deps
MATURIN_OPTS = --manylinux 2014 --auditwheel skip

# Build options
TARGET ?= $(shell rustc -vV | awk '/^host:/ {print $$2}')
BUILD_OPTS = --release --target $(TARGET)

ifneq (,$(findstring windows,$(TARGET)))
CARGO_FEATURES = --features sdl2_bundle
else ifneq (,$(findstring darwin,$(TARGET)))
CARGO_FEATURES = --features sdl2_bundle
else ifneq (,$(findstring emscripten,$(TARGET)))
CARGO_FEATURES = --features html5
else
CARGO_FEATURES = --features sdl2
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
	@cd $(RUST_DIR); cargo clippy $(CLIPPY_OPTS)
	@cd $(RUST_DIR); cargo clippy --target $(WASM_TARGET) $(CLIPPY_OPTS)
	@ruff check $(ROOT_DIR)

update:
	@rustup -q update
	@cargo -q install cargo-outdated
	@cd $(RUST_DIR); cargo -q update
	@cd $(RUST_DIR); cargo -q outdated --root-deps-only
	@pip3 -q install -U -r $(PYTHON_DIR)/requirements.txt

format:
	@cd $(RUST_DIR); cargo fmt -- --emit=files
	@ruff format $(ROOT_DIR)

build: format
	@rustup target add $(TARGET)
	@$(SCRIPTS_DIR)/generate_readme_abspath
	@cp LICENSE $(PYTHON_DIR)/pyxel
	@cd $(PYTHON_DIR); maturin build -o ../$(DIST_DIR) $(BUILD_OPTS) $(MATURIN_OPTS) $(CARGO_FEATURES)

install: build
	@pip3 install --force-reinstall `ls -rt $(DIST_DIR)/*.whl | tail -n 1`

test: install
	@cd $(RUST_DIR); cargo test $(BUILD_OPTS) $(CARGO_FEATURES)
	@python3 -m unittest discover $(RUST_DIR)/pyxel-wrapper/tests

	@bash -c 'set -e; trap "exit 130" INT; for f in $(EXAMPLES_DIR)/*.py; do pyxel run "$$f"; done'
	@bash -c 'set -e; trap "exit 130" INT; for f in $(EXAMPLES_DIR)/*.pyxapp; do pyxel play "$$f"; done'
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
	@make clean TARGET=$(WASM_TARGET)

build-wasm:
	@embuilder build sdl2 --pic
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@make build TARGET=$(WASM_TARGET)
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
