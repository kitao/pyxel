ROOT_DIR = ..
PYXEL_DIR = $(ROOT_DIR)/pyxel
PYXEL_LIB_DIR = $(PYXEL_DIR)/lib/$(PLATFORM)
PYXEL_LIB_NAME = pyxel_wrapper
LIB_DIR = $(ROOT_DIR)/lib
TESTS_DIR = $(LIB_DIR)/wrapper/tests
EXAMPLES_DIR = $(PYXEL_DIR)/examples
DIST_DIR = $(ROOT_DIR)/dist
DIST_NAME = $(shell echo $(WHEEL_NAME) | sed -E 's/.*(pyxel-.*)-py3.*/\1/')-$(PLATFORM)
WHEEL_NAME = $(shell ls $(DIST_DIR)/pyxel-*.whl)
PYOXIDIZER_DIR = $(LIB_DIR)/build

ifneq ($(findstring _NT-,$(shell uname)),)
# Windows
PLATFORM = windows
PYTHON = python
SRC_LIB_NAME = $(PYXEL_LIB_NAME).dll
DST_LIB_NAME = $(PYXEL_LIB_NAME).pyd
SRC_SDL2_DLL = $(SRC_LIB_DIR)/SDL2.dll
DST_SDL2_DLL = $(DST_LIB_DIR)/SDL2.dll
COPY_SDL2_DLL = if [ ! -f $(DST_SDL2_DLL) ]; then cp $(SRC_SDL2_DLL) $(DST_SDL2_DLL); fi
COPY_PYXEL_VBS = cp pyxel.vbs $(DIST_NAME)
else ifeq ($(shell uname),Darwin)
# MacOS
PLATFORM = macos
PYTHON = python3
SRC_LIB_NAME = lib$(PYXEL_LIB_NAME).dylib
DST_LIB_NAME = $(PYXEL_LIB_NAME).so
else ifeq ($(shell uname),Linux)
# Linux
PLATFORM = linux
PYTHON = python3
SRC_LIB_NAME = lib$(PYXEL_LIB_NAME).so
DST_LIB_NAME = $(PYXEL_LIB_NAME).so
else
# Others
$(error "unsupported platform")
endif

ifneq ($(DEBUG),)
# Debug
CARGO_BUILD_OPT =
CARGO_TARGET_DIR = $(LIB_DIR)/target/debug
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(CARGO_TARGET_DIR)
TEST_IMPORT_DIR = $(TESTS_DIR)/debug_module
else
# Release
CARGO_BUILD_OPT = --release
CARGO_TARGET_DIR = $(LIB_DIR)/target/release
SRC_LIB_DIR = $(CARGO_TARGET_DIR)
DST_LIB_DIR = $(PYXEL_LIB_DIR)
TEST_IMPORT_DIR = $(ROOT_DIR)
endif

.PHONY: all clean format build test wheel dist

all: build

clean:
	@cargo clean
	@rm -f $(PYXEL_LIB_DIR)/* $(DIST_DIR)/*.whl

format:
	@cargo +nightly fmt -- --emit=files
	@cargo clippy -- --no-deps
	@isort $(ROOT_DIR) && black $(ROOT_DIR)
	@flake8 $(ROOT_DIR)/*.py $(PYXEL_DIR)

build:
	@cargo build $(CARGO_BUILD_OPT)
	@mkdir -p $(DST_LIB_DIR) && cp $(SRC_LIB_DIR)/$(SRC_LIB_NAME) $(DST_LIB_DIR)/$(DST_LIB_NAME)
	@$(COPY_SDL2_DLL)

test: build
	@cargo test
	@PYTHONPATH=$(TEST_IMPORT_DIR) $(PYTHON) -m unittest discover $(TESTS_DIR)
	@trap break INT; for example in $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py); do \
		PYTHONPATH=$(TEST_IMPORT_DIR) $(PYTHON) $$example; \
	done; trap - INT

wheel:
	@cd $(ROOT_DIR) && $(PYTHON) setup.py bdist_wheel
	@rm -rf $(ROOT_DIR)/build $(ROOT_DIR)/*.egg-info

dist:
	@rm -rf $(PYOXIDIZER_DIR)
	@pyoxidizer build --release --var wheel_name $(WHEEL_NAME)
	@mv $(PYOXIDIZER_DIR)/*/release/install $(DIST_NAME)
	@$(COPY_PYXEL_VBS)
	@zip -r $(DIST_NAME).zip $(DIST_NAME)
	@mv -f $(DIST_NAME).zip $(DIST_DIR)
	@mv $(DIST_NAME) $(PYOXIDIZER_DIR)
