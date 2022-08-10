#
# [How to build]
#
# Required tools:
#	- cmake
#	- rustup
#	- python 3.7+
#	- Emscripten (only for WASM build)
#
#	[Windows]
#
#	[Linux]
#	- SDL2 (e.g. libsdl2-dev for Ubuntu)
#
# Advance preparation:
#	rustup install nightly
#	python3 -m venv .venv
#	source .venv/bin/activate
#	pip install -U pip
#	pip install -r requirements.txt
#
# Format and lint the code:
#	make format
#
# Build the package
#	make clean build
#
# Install the package:
#	make install
#
# Build and install the package:
#	make clean all
#
# Build, install, and test the package:
#	make clean test
#
# Build for the specified target:
#	make clean build TARGET=target_name
#
# Build for the specified target with Nightly Rust
#	make clean build NIGHTLY=1 TARGET=target_name
#

ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
PYXEL_DIR = $(ROOT_DIR)/python/pyxel
CRATES_DIR = $(ROOT_DIR)/crates
SCRIPTS_DIR = $(ROOT_DIR)/scripts
EXAMPLES_DIR = $(PYXEL_DIR)/examples
CRATES = $(wildcard $(CRATES_DIR)/*)
EXAMPLES = $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py)

ifneq ($(NIGHTLY),)
RUST_ENV = RUSTUP_TOOLCHAIN=nightly
endif

ifneq ($(TARGET),)
ADD_TARGET = $(RUST_ENV) rustup target add $(TARGET)
TARGET_OPT = --target $(TARGET)
endif

.PHONY: all format clean build install test wasm-clean wasm-build

all: build install

format:
	@for crate in $(CRATES); do \
		cd $$crate; \
		cargo +nightly fmt -- --emit=files; \
		cargo clippy -- --no-deps; \
		cd -; \
	done
	@isort $(ROOT_DIR) && black $(ROOT_DIR)
	@flake8 $(ROOT_DIR)/*.py $(PYXEL_DIR)

clean:
	@for crate in $(CRATES); do \
		cd $$crate; \
		$(RUST_ENV) cargo clean $(TARGET_OPT); \
		cd -; \
	done

build:
	@$(ADD_TARGET)
	@$(SCRIPTS_DIR)/update_readme
	@$(RUST_ENV) maturin build --release $(TARGET_OPT)
	@mkdir -p $(DIST_DIR)
	@cp $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl $(DIST_DIR)

install:
	@pip install --force-reinstall $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl

test: build install
	@cd $(CRATES_DIR)/pyxel-engine; cargo test --release
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

wasm-clean:
	@make clean NIGHTLY=1 TARGET=wasm32-unknown-emscripten

wasm-build:
	@make build NIGHTLY=1 TARGET=wasm32-unknown-emscripten
