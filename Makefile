#
# [How to build]
#
# Required tools:
#	- cmake
#	- rustup
#	- python 3.7+
#
#	[Windows]
#	- TBD
#
#	[Linux]
#	- SDL2 (e.g. libsdl2-dev for Ubuntu)
#
#	[WASM]
#	- Emscripten
#
# Advance preparation:
#	rustup install nightly
#	scripts/setup_venv
#	source .venv/vin/activate
#
# Build the package in the dist directory
#	make clean build
#
# Build the package for the specified target:
#	make clean build TARGET=target_triple
#
# Build the package and install it in the current venv:
#	make clean all
#
# Build and test the package in the current venv:
#	make clean test
#
# Build the package for WASM in the dist directory
#	make clean-wasm build-wasm
#

ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
PYXEL_DIR = $(ROOT_DIR)/python/pyxel
CRATES_DIR = $(ROOT_DIR)/crates
SCRIPTS_DIR = $(ROOT_DIR)/scripts
EXAMPLES_DIR = $(PYXEL_DIR)/examples
CRATES = $(wildcard $(CRATES_DIR)/*)
EXAMPLES = $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py)

ifeq ($(TARGET),)
ADD_TARGET =
BUILD_OPTS = --release
else
ADD_TARGET = rustup target add $(TARGET)
BUILD_OPTS = --release --target $(TARGET)
endif

WASM_ENVVARS = RUSTUP_TOOLCHAIN=nightly MATURIN_EMSCRIPTEN_VERSION=3.1.14
WASM_TARGET = wasm32-unknown-emscripten

.PHONY: all clean distclean format build install test clean-wasm build-wasm

all: build install

clean:
	@for crate in $(CRATES); do \
		cd $$crate; \
		cargo clean $(BUILD_OPTS); \
		cd -; \
	done

distclean:
	@for crate in $(CRATES); do \
		rm -rf $$crate/target; \
	done
	@rm -rf $(DIST_DIR)

format:
	@for crate in $(CRATES); do \
		cd $$crate; \
		cargo +nightly fmt -- --emit=files; \
		cd -; \
	done
	@isort $(ROOT_DIR)
	@black $(ROOT_DIR)
	@$(SCRIPTS_DIR)/update_readme

build: format
	@$(ADD_TARGET)
	@rm -f $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl
	@maturin build $(BUILD_OPTS)
	@mkdir -p $(DIST_DIR)
	@cp $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl $(DIST_DIR)

install:
	@pip install --force-reinstall $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl

test: build install
	@cd $(CRATES_DIR)/pyxel-engine; cargo test $(BUILD_OPTS)
	@python -m unittest discover $(CRATES_DIR)/pyxel-wrapper/tests

	@for example in $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py); do \
		pyxel run $$example; \
	done
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
