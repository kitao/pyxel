ROOT_DIR = ..
DIST_DIR = $(ROOT_DIR)/dist
PYXEL_DIR = $(ROOT_DIR)/pyxel
CRATES_DIR = $(ROOT_DIR)/crates
LIB_DIR = $(PYXEL_DIR)/lib/$(OS_NAME)
LIB_NAME = pyxel_wrapper
TESTS_DIR = $(CRATES_DIR)/wrapper/tests
EXAMPLES_DIR = $(PYXEL_DIR)/examples

ifeq ($(shell uname),Darwin)
OS_NAME = macos
SRC_LIB_EXT = .dylib
DST_LIB_EXT = .so
endif
ifeq ($(shell uname),Linux)
OS_NAME = linux
SRC_LIB_EXT = .so
DST_LIB_EXT = .so
endif
ifneq ($(findstring MINGW64,$(shell uname)),)
OS_NAME = windows
SRC_LIB_EXT = .dll
DST_LIB_EXT = .pyd
endif

ifeq ($(RELEASE),)
CARGO_BUILD_OPT =
CARGO_TARGET_DIR = $(CRATES_DIR)/target/debug
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(CARGO_TARGET_DIR)
TEST_IMPORT_DIR = $(TESTS_DIR)/debug_module
else
CARGO_BUILD_OPT = --release
CARGO_TARGET_DIR = $(CRATES_DIR)/target/release
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(LIB_DIR)
TEST_IMPORT_DIR = $(ROOT_DIR)
endif

SRC_LIB = $(SRC_LIB_DIR)/lib$(LIB_NAME)$(SRC_LIB_EXT)
DST_LIB = $(DST_LIB_DIR)/$(LIB_NAME)$(DST_LIB_EXT)

.PHONY: all build clean dist format lint test

all: build

build:
	@cargo build $(CARGO_BUILD_OPT)
	@mkdir -p $(DST_LIB_DIR) && cp $(SRC_LIB) $(DST_LIB)

dist:
	@cd ..; python3 setup.py bdist_wheel
	$(eval DIST_NAME := $(shell ls $(DIST_DIR)/pyxel-*.whl | sed -E 's/.*(pyxel-.*)-py3.*/\1/')-$(OS_NAME))
	@rm -rf build
	@pyoxidizer build --release
	@cp -r build/*/release/install $(DIST_DIR)/$(DIST_NAME)
	@rm -f $(DIST_DIR)/$(DIST_NAME).zip
	@cd $(DIST_DIR); zip -r $(DIST_NAME).zip $(DIST_NAME)
	@rm -rf $(DIST_DIR)/$(DIST_NAME)

clean:
	@cargo clean
	@rm -rf $(LIB_DIR)

format:
	@cargo +nightly fmt -- --emit=files
	@isort $(ROOT_DIR) && black $(ROOT_DIR)

lint: format
	@cargo clippy
	@flake8 $(ROOT_DIR)/*.py $(PYXEL_DIR)

test: build
	@cargo test
	@PYTHONPATH=$(TEST_IMPORT_DIR) python3 -m unittest discover $(TESTS_DIR)
	@trap break INT; for example in $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py); do \
		PYTHONPATH=$(TEST_IMPORT_DIR) python3 $$example; \
	done; trap - INT
