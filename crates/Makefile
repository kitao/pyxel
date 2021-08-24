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

PYXEL_LIB_DIR = ../../pyxel/lib/$(PLATFORM_NAME)
PYXEL_EXAMPLES_DIR = ../pyxel/examples
CARGO_DEBUG_DIR = target/debug
CARGO_RELEASE_DIR = target/release
SRC_LIB_NAME = libpyxel_wrapper.$(SRC_LIB_EXT)
DST_LIB_NAME = pyxel_wrapper.$(DST_LIB_EXT)

ifeq ($(RELEASE),)
CARGO_BUILD_OPT =
SRC_LIB_PATH = $(CARGO_DEBUG_DIR)/$(SRC_LIB_NAME)
DST_LIB_PATH = $(CARGO_DEBUG_DIR)/$(DST_LIB_NAME)
TEST_IMPORT_DIR = wrapper
else
CARGO_BUILD_OPT = --release
SRC_LIB_PATH = $(CARGO_RELEASE_DIR)/$(SRC_LIB_NAME)
DST_LIB_PATH = $(PYXEL_LIB_DIR)/$(DST_LIB_NAME)
TEST_IMPORT_DIR = ..
endif

.PHONY: all build test format clean

all: build

build:
	cargo build $(CARGO_BUILD_OPT)
	cp $(SRC_LIB_PATH) $(DST_LIB_PATH)

test:
	# cargo test
	# python3 wrapper/tests/main.py
	PYTHONPATH=$(TEST_IMPORT_DIR) python3 $(PYXEL_EXAMPLES_DIR)/01_hello_pyxel.py

format:
	cargo fmt -- --emit=files

clean:
	cargo clean
	rm -rf $(PYXEL_LIB_DIR)
