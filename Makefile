#
# [How to build]
#
# Required tools:
#	[Common]
#	- cmake
#	- rustup
#	- python 3.7+
#
#	[Windows]
#	- Build Tools for Visual Studio 2019
#	- Cygwin64 with make and zip added
#
#	[Linux]
#	- SDL2 package (e.g. libsdl2-dev for Ubuntu)
#
# Advance preparation:
#	rustup install nightly
#	python3 -m venv .venv
#	source .venv/bin/activate
#	pip install -r requirements.txt
#
# Format and lint the code:
#	make format
#
# Build the package:
#	make clean all
#
# Build and test the package:
#	make clean test
#
# Install the package:
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
