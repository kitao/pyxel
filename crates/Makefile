ifeq ($(shell uname),Darwin)
PLATFORM_NAME = mac
SRC_LIB_EXT = dylib
DST_LIB_EXT = so
endif
ifeq ($(shell uname),Linux)
PLATFORM_NAME = linux
SRC_LIB_EXT = so
DST_LIB_EXT = so
endif
ifneq ($(findstring MINGW64,$(shell uname)),)
PLATFORM_NAME = windows
SRC_LIB_EXT = dll
DST_LIB_EXT = pyd
endif

PYXEL_LIB_DIR = ../pyxel/lib/$(PLATFORM_NAME)
PYXEL_TESTS_DIR = wrapper/tests
PYXEL_EXAMPLES_DIR = ../pyxel/examples
CARGO_DEBUG_DIR = target/debug
CARGO_RELEASE_DIR = target/release
SRC_LIB_NAME = libpyxel_wrapper.$(SRC_LIB_EXT)
DST_LIB_NAME = pyxel_wrapper.$(DST_LIB_EXT)

ifeq ($(RELEASE),)
CARGO_BUILD_OPT =
SRC_LIB_DIR = $(CARGO_DEBUG_DIR)
DST_LIB_DIR = $(CARGO_DEBUG_DIR)
TEST_IMPORT_DIR = $(PYXEL_TESTS_DIR)
else
CARGO_BUILD_OPT = --release
SRC_LIB_DIR = $(CARGO_RELEASE_DIR)
DST_LIB_DIR = $(PYXEL_LIB_DIR)
TEST_IMPORT_DIR = ..
endif

.PHONY: all build test format clean

all: build

build: format
	cargo build $(CARGO_BUILD_OPT)
	mkdir -p $(DST_LIB_DIR)
	cp $(SRC_LIB_DIR)/$(SRC_LIB_NAME) $(DST_LIB_DIR)/$(DST_LIB_NAME)

test: build
	# cargo test
	# python3 $(PYXEL_TESTS_DIR)/main.py
	cd $(PYXEL_TESTS_DIR); PYTHONPATH=$(TEST_IMPORT_DIR) python3 -m unittest
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/01_hello_pyxel.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/02_jump_game.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/03_draw_api.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/04_sound_api.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/05_color_palette.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/06_click_game.py

format:
	cargo fmt -- --emit=files

clean:
	cargo clean
	rm -rf $(PYXEL_LIB_DIR)
