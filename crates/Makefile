PYXEL_LIB_DIR = ../pyxel/lib/$(OS_NAME)
PYXEL_LIB_NAME = pyxel_extension
PYXEL_BIN_DIR = ../pyxel/bin/$(OS_NAME)
PYXEL_BIN_NAME = pyxel
PYXEL_TESTS_DIR = extension/tests
PYXEL_EXAMPLES_DIR = ../pyxel/examples

ifeq ($(shell uname),Darwin)
OS_NAME = macos
SRC_LIB_EXT = .dylib
DST_LIB_EXT = .so
BIN_EXT =
endif
ifeq ($(shell uname),Linux)
OS_NAME = linux
SRC_LIB_EXT = .so
DST_LIB_EXT = .so
BIN_EXT =
endif
ifneq ($(findstring MINGW64,$(shell uname)),)
OS_NAME = windows
SRC_LIB_EXT = .dll
DST_LIB_EXT = .pyd
BIN_EXT = .exe
endif

ifeq ($(RELEASE),)
CARGO_BUILD_OPT =
CARGO_TARGET_DIR = target/debug
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(CARGO_TARGET_DIR)
SRC_BIN_DIR = $(CARGO_TARGET_DIR)
DST_BIN_DIR = $(CARGO_TARGET_DIR)
TEST_IMPORT_DIR = $(PYXEL_TESTS_DIR)/debug_module
else
CARGO_BUILD_OPT = --release
CARGO_TARGET_DIR = target/release
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(PYXEL_LIB_DIR)
SRC_BIN_DIR = $(CARGO_TARGET_DIR)
DST_BIN_DIR = $(PYXEL_BIN_DIR)
TEST_IMPORT_DIR = ..
endif

SRC_LIB = $(SRC_LIB_DIR)/lib$(PYXEL_LIB_NAME)$(SRC_LIB_EXT)
DST_LIB = $(DST_LIB_DIR)/$(PYXEL_LIB_NAME)$(DST_LIB_EXT)
SRC_BIN = $(SRC_BIN_DIR)/$(PYXEL_BIN_NAME)$(BIN_EXT)
DST_BIN = $(DST_BIN_DIR)/$(PYXEL_BIN_NAME)$(BIN_EXT)

.PHONY: all build test format lint clean

all: build

build: format
	@cargo build $(CARGO_BUILD_OPT)
	@mkdir -p $(DST_LIB_DIR) $(DST_BIN_DIR)
	@if [ "$(SRC_LIB)" != "$(DST_LIB)" ]; then cp $(SRC_LIB) $(DST_LIB); fi
	@if [ "$(SRC_BIN)" != "$(DST_BIN)" ]; then cp $(SRC_BIN) $(DST_BIN); fi

test: build
	@cargo test
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 -m unittest discover $(PYXEL_TESTS_DIR)
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/01_hello_pyxel.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/02_jump_game.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/03_draw_api.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/04_sound_api.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/05_color_palette.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/06_click_game.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/07_snake.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/08_triangle_api.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/09_shooter.py
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/10_platformer.py

format:
	@cargo fmt -- --emit=files

lint:
	@cargo clippy

clean:
	@cargo clean
	@rm -rf $(PYXEL_LIB_DIR) $(PYXEL_BIN_DIR)
