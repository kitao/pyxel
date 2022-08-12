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
#	- TBD
#
#	[Linux]
#	- SDL2 (e.g. libsdl2-dev for Ubuntu)
#
# Advance preparation:
#	rustup install nightly
#	scripts/setup_venv
#	source .venv/vin/activate
#
# Build the package in the dist directory
#	make build
#
# Build the package and install it in the current venv:
#	make all
#
# Build and test the package in the current venv:
#	make test
#
# Build the package for WASM in the dist directory
#	make wasm-build
#
# Build the package for the specified target:
#	make build TARGET=target_triple
#
# Build the package for the specified target with Nightly Rust
#	make build NIGHTLY=1 TARGET=target_triple
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
else
RUST_ENV =
endif

ifneq ($(TARGET),)
ADD_TARGET = $(RUST_ENV) rustup target add $(TARGET)
RUST_ARGS = --release --target $(TARGET)
else
RUST_ARGS = --release
endif

.PHONY: all clean distclean format build install test wasm-clean wasm-build

all: build install

clean:
	@for crate in $(CRATES); do \
		cd $$crate; \
		$(RUST_ENV) cargo clean $(RUST_ARGS); \
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
	@$(RUST_ENV) maturin build $(RUST_ARGS)
	@mkdir -p $(DIST_DIR)
	@cp $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl $(DIST_DIR)

install:
	@pip install --force-reinstall $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl

test: build install
	@cd $(CRATES_DIR)/pyxel-engine; $(RUST_ENV) cargo test $(RUST_ARGS)
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
