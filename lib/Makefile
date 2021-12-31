ROOT_DIR = ..
PYXEL_DIR = $(ROOT_DIR)/pyxel
PYXEL_LIB_DIR = $(PYXEL_DIR)/lib/$(PLATFORM)
PYXEL_LIB_NAME = pyxel_wrapper
BUILD_DIR = $(ROOT_DIR)/lib
BUILD_OPT = --release
TESTS_DIR = $(BUILD_DIR)/wrapper/tests
EXAMPLES_DIR = $(PYXEL_DIR)/examples
TEST_RESOURCE = $(EXAMPLES_DIR)/assets/resource_example.pyxres;
DIST_DIR = $(ROOT_DIR)/dist
DIST_NAME = $(shell echo $(WHEEL_NAME) | sed -E 's/.*(pyxel-.*)-py3.*/\1/')-$(PLATFORM)
WHEEL_NAME = $(shell ls $(DIST_DIR)/pyxel-*.whl)
PYOXIDIZER_DIR = $(BUILD_DIR)/build

# Windows
ifneq ($(findstring _NT-,$(shell uname)),)
PLATFORM = windows
PYTHON = python
TARGET_DIR = $(BUILD_DIR)/target/release
SRC_LIB_FILE = $(TARGET_DIR)/$(PYXEL_LIB_NAME).dll
DST_LIB_FILE = $(PYXEL_LIB_DIR)/$(PYXEL_LIB_NAME).pyd
ISCC = 'iscc'
ISCC_FILE = $(BUILD_DIR)/setup.iss

CARGO_CLEAN = cargo clean
CARGO_BUILD = cargo build $(BUILD_OPT)
COPY_FILES = \
	cp $(SRC_LIB_FILE) $(DST_LIB_FILE) && \
	cp -f $(TARGET_DIR)/SDL2.dll $(PYXEL_LIB_DIR)
COPY_VBS = cp -f $(BUILD_DIR)/pyxel.vbs $(DIST_NAME)
BUILD_INSTALLER = $(ISCC) $(ISCC_FILE)

# MacOS
else ifeq ($(shell uname),Darwin)
PLATFORM = macos
PYTHON = python3
TARGET1 = x86_64-apple-darwin
TARGET2 = aarch64-apple-darwin
TARGET_DIR1 = $(BUILD_DIR)/target/$(TARGET1)/release
TARGET_DIR2 = $(BUILD_DIR)/target/$(TARGET2)/release
SRC_LIB_FILE1 = $(TARGET_DIR1)/lib$(PYXEL_LIB_NAME).dylib
SRC_LIB_FILE2 = $(TARGET_DIR2)/lib$(PYXEL_LIB_NAME).dylib
DST_LIB_FILE = $(PYXEL_LIB_DIR)/$(PYXEL_LIB_NAME).so

CARGO_VARS = \
	SDKROOT=$(shell xcrun -sdk macosx --show-sdk-path) \
	MACOSX_DEPLOYMENT_TARGET=$(shell xcrun -sdk macosx --show-sdk-platform-version)
CARGO_CLEAN = \
	cargo clean --target=$(TARGET1) && \
	cargo clean --target=$(TARGET2)
CARGO_BUILD = \
	rustup target add $(TARGET1) && \
	$(CARGO_VARS) cargo build $(BUILD_OPT) --target=$(TARGET1) && \
	rustup target add $(TARGET2) && \
	$(CARGO_VARS) cargo build $(BUILD_OPT) --target=$(TARGET2)
COPY_FILES = lipo -create -output $(DST_LIB_FILE) $(SRC_LIB_FILE1) $(SRC_LIB_FILE2)

# Linux
else ifeq ($(shell uname),Linux)
PLATFORM = linux
PYTHON = python3
TARGET_DIR = $(BUILD_DIR)/target/release
SRC_LIB_FILE = $(TARGET_DIR)/lib$(PYXEL_LIB_NAME).so
DST_LIB_FILE = $(PYXEL_LIB_DIR)/$(PYXEL_LIB_NAME).so

CARGO_CLEAN = cargo clean
CARGO_BUILD = cargo build $(BUILD_OPT)
COPY_FILES = cp $(SRC_LIB_FILE) $(DST_LIB_FILE)

# Others
else
$(error "unsupported platform")
endif

.PHONY: all clean format build test wheel dist

all: build

clean:
	@$(CARGO_CLEAN)
	@rm -f $(PYXEL_LIB_DIR)/* $(DIST_DIR)/*.whl

format:
	@cargo +nightly fmt -- --emit=files
	@cargo clippy -- --no-deps
	@isort $(ROOT_DIR) && black $(ROOT_DIR)
	@flake8 $(ROOT_DIR)/*.py $(PYXEL_DIR)

build:
	$(CARGO_BUILD)
	@$(COPY_FILES)

test: build
	@cargo test
	@PYTHONPATH=$(ROOT_DIR) $(PYTHON) -m unittest discover $(TESTS_DIR)
	@trap break INT; for example in $(wildcard $(EXAMPLES_DIR)/[0-9][0-9]_*.py); do \
		PYTHONPATH=$(ROOT_DIR) $(PYTHON) -m pyxel run $$example; \
	done; trap - INT
	PYTHONPATH=$(ROOT_DIR) $(PYTHON) -m pyxel edit $(TEST_RESOURCE)

wheel:
	@cd $(ROOT_DIR) && $(PYTHON) setup.py bdist_wheel
	@rm -rf $(ROOT_DIR)/build $(ROOT_DIR)/*.egg-info

dist:
	@rm -rf $(PYOXIDIZER_DIR) $(DIST_NAME) $(DIST_NAME).zip
	@pyoxidizer build --release --var wheel_name $(WHEEL_NAME)
	@mv $(PYOXIDIZER_DIR)/*/release/install $(DIST_NAME)
	@$(COPY_VBS)
	@zip -r $(DIST_NAME).zip $(DIST_NAME)
	@mv -f $(DIST_NAME).zip $(DIST_DIR)
	@mv $(DIST_NAME) $(PYOXIDIZER_DIR)
	@$(BUILD_INSTALLER)
