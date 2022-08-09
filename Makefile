#
# [How to build]
#
# Common prerequisites:
#	- cmake
#	- rustup
#	- python3
#
# Prerequisites for Windows:
#	- Build Tools for Visual Studio 2019
#	- Cygwin64 with make and zip added
#
# Prerequisites for Linux:
#	- SDL2 package (libsdl2-dev for Ubuntu)
#
# Format and lint code:
#	(After `rustup install nightly`)
#	make format
#
# Build:
#	(After activating the virtual environment)
#	make clean all
#
# Build and test:
#	(After activating the virtual environment)
#	make clean test
#
# Install Pyxel after build:
#	(After deactivating the virtual environment)
#	pip3 install dist/*.whl
#	(Prefix `sudo` may be needed on Linux)
#

ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
PYXEL_DIR = $(ROOT_DIR)/python/pyxel
CRATES_DIR = $(ROOT_DIR)/crates
EXAMPLES_DIR = $(PYXEL_DIR)/examples
CRATES = $(wildcard $(CRATES_DIR)/*)
EXAMPLES = $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py)

.PHONY: all clean format build test

all: build

clean:
	@for crate in $(CRATES); do \
		cd $$crate; \
		cargo clean; \
		cd -; \
	done
	@rm -rf $(DIST_DIR)

format:
	@for crate in $(CRATES); do \
		cd $$crate; \
		cargo +nightly fmt -- --emit=files; \
		cargo clippy -- --no-deps; \
		cd -; \
	done
	@isort $(ROOT_DIR) && black $(ROOT_DIR)
	@flake8 $(ROOT_DIR)/*.py $(PYXEL_DIR)

build:
	maturin build --release
	mkdir -p $(DIST_DIR)
	cp $(CRATES_DIR)/pyxel-wrapper/target/wheels/*.whl $(DIST_DIR)
	pip install --force-reinstall $(DIST_DIR)/*.whl

test: build
	cd $(CRATES_DIR)/pyxel-engine; cargo test --release
	python -m unittest discover $(CRATES_DIR)/pyxel-wrapper/tests

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
