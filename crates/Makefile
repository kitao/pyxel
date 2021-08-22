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

PYTHON_LIB_DIR = ../../pyxel/lib/$(PLATFORM_NAME)
DEBUG_TARGET_DIR = target/debug
RELEASE_TARGET_DIR = target/release
SRC_LIB_NAME = libpyxel_extension.$(SRC_LIB_EXT)
DST_LIB_NAME = pyxel_extension.$(DST_LIB_EXT)

.PHONY: all debug release test clean

all: debug

debug:
	cargo build
	cp $(DEBUG_TARGET_DIR)/$(SRC_LIB_NAME) $(DEBUG_TARGET_DIR)/$(DST_LIB_NAME)

release:
	cargo build --release
	cp $(RELEASE_TARGET_DIR)/$(SRC_LIB_NAME) $(PYTHON_LIB_DIR)/$(DST_LIB_NAME)

test:
	cargo test
	python3 pyxel-wrapper/tests/main.py

clean:
	cargo clean
	rm -rf $(PYTHON_LIB_DIR)
