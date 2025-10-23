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
#	- libsdl2-dev 2.32.0
#
#	[Web]
#	- Pyodide-customized version of Emscripten 4.0.9
#	  To build it, run the following commands:
#		git clone --branch 0.29.0 --depth 1 https://github.com/pyodide/pyodide.git pyodide
#		cd pyodide/emsdk
#		CMAKE_POLICY_VERSION_MINIMUM=3.5 make
#		source pyodide/emsdk/emsdk/emsdk_env.sh
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
TARGET ?= $(shell rustc -vV | awk '/^host:/ {print $$2}')
WASM_TARGET = wasm32-unknown-emscripten

# Build options
ifeq ($(TARGET),$(WASM_TARGET))
RUSTFLAGS += \
	-C panic=abort \
    -C link-arg=-fwasm-exceptions \
    -C link-arg=-sSIDE_MODULE=2 \
    -C link-arg=-lSDL2 \
    -C link-arg=-lhtml5
endif

CARGO_OPTS = --release --target $(TARGET) -Zbuild-std=std,panic_abort

ifneq (,$(or $(findstring windows,$(TARGET)),$(findstring darwin,$(TARGET))))
CARGO_OPTS += --features sdl2_bundle
else
CARGO_OPTS += --features sdl2
endif

# Tool options
CLIPPY_OPTS = -q -- --no-deps
MATURIN_OPTS = --manylinux 2014 --auditwheel skip


.PHONY: \
	all clean distclean update format lint build install test \
	clean-wasm lint-wasm build-wasm start-test-server test-wasm \
	setup-wasm-github test-wasm-github

all: build

clean:
	@cd $(RUST_DIR); cargo clean --target $(TARGET)

distclean:
	@rm -rf $(DIST_DIR)
	@rm -rf $(RUST_DIR)/target

update:
	@rustup -q update
	@cargo -q install cargo-outdated
	@cd $(RUST_DIR); cargo -q update
	@cd $(RUST_DIR); cargo -q outdated --root-deps-only
	@pip3 -q install -U -r $(PYTHON_DIR)/requirements.txt

format:
	@cd $(RUST_DIR); cargo fmt -- --emit=files
	@ruff format $(ROOT_DIR)

lint:
	@cd $(RUST_DIR); cargo clippy $(CARGO_OPTS) $(CLIPPY_OPTS) || true
	@ruff check $(ROOT_DIR) || true

build: format lint
	@rustup component add rust-src
	@rustup target add $(TARGET)
	@$(SCRIPTS_DIR)/generate_readme_abspath
	@cp LICENSE $(PYTHON_DIR)/pyxel
	@cd $(PYTHON_DIR); RUSTFLAGS="$(RUSTFLAGS)" maturin build -o ../$(DIST_DIR) $(CARGO_OPTS) $(MATURIN_OPTS)

install: build
	@pip3 install --force-reinstall `ls -rt $(DIST_DIR)/*.whl | tail -n 1`

test: install
	@cd $(RUST_DIR); cargo test $(CARGO_OPTS)

	@bash -c 'set -e; trap "exit 130" INT; for f in $(EXAMPLES_DIR)/*.py; do pyxel run "$$f"; done'
	@bash -c 'set -e; trap "exit 130" INT; for f in $(EXAMPLES_DIR)/apps/*.pyxapp; do pyxel play "$$f"; done'
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
	@$(MAKE) clean TARGET=$(WASM_TARGET)

lint-wasm:
	@$(MAKE) lint TARGET=$(WASM_TARGET)

build-wasm:
	@embuilder build sdl2 --pic
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@$(MAKE) build TARGET=$(WASM_TARGET)
	@$(SCRIPTS_DIR)/install_wasm_wheel

start-test-server:
	$(SCRIPTS_DIR)/switch_html_scripts local
	@bash -c "trap '$(SCRIPTS_DIR)/switch_html_scripts cdn' INT TERM; $(SCRIPTS_DIR)/start_test_server"

test-wasm: build-wasm start-test-server

setup-wasm-github:
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@$(SCRIPTS_DIR)/download_wasm_wheel
	@$(SCRIPTS_DIR)/install_wasm_wheel

test-wasm-github: setup-wasm-github start-test-server
