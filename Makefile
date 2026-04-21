#
# Prerequisites:
#   - git, make, cmake, rustup, python 3.10+, node.js
#   - Windows: Git Bash
#   - Linux: python3-pip, python3-venv, clang, libclang-dev, libsdl2-dev 2.32.0
#   - ./scripts/setup_venv
#
# Each new shell:
#   - macOS/Linux: source .venv/bin/activate
#   - Windows (Git Bash): source .venv/Scripts/activate
#
# Native:
#   - Lint: make lint
#   - Build: make clean build
#   - Test (unit): make test-unit
#   - Test (full): make clean test (includes watch)
#
# WASM:
#   - Setup once:
#       git clone --branch 0.29.3 --depth 1 https://github.com/pyodide/pyodide.git pyodide
#       cd pyodide/emsdk
#       CMAKE_POLICY_VERSION_MINIMUM=3.5 make
#   - Each new shell before WASM commands:
#       source pyodide/emsdk/emsdk_env.sh
#   - Lint: make lint-wasm
#   - Build/Test:
#       make clean-wasm build-wasm
#       make clean-wasm test-wasm
#
# Web pages:
#   - Setup once: cd web && npm install
#   - Build: make pages
#

# Project directories
ROOT_DIR := .
DIST_DIR := $(ROOT_DIR)/dist
CRATES_DIR := $(ROOT_DIR)/crates
PYTHON_DIR := $(ROOT_DIR)/python
SCRIPTS_DIR := $(ROOT_DIR)/scripts

# Build targets
TARGET ?= $(shell rustc -vV | awk '/^host:/ {print $$2}')
WASM_TARGET := wasm32-unknown-emscripten

# WASM path remap flags
REMAP_SRC_PATH := $(abspath $(ROOT_DIR))
REMAP_USER_HOME ?= /user
RUST_REMAP_FLAGS := --remap-path-prefix=$(REMAP_SRC_PATH)=/src/pyxel
WASM_PREFIX_MAP_FLAGS := -ffile-prefix-map=$(REMAP_SRC_PATH)=/src/pyxel
ifneq ($(HOME),)
RUST_REMAP_FLAGS += --remap-path-prefix=$(HOME)=$(REMAP_USER_HOME)
WASM_PREFIX_MAP_FLAGS += -ffile-prefix-map=$(HOME)=$(REMAP_USER_HOME)
endif

# Build options
ifeq ($(TARGET),$(WASM_TARGET))
RUSTFLAGS += \
	$(RUST_REMAP_FLAGS) \
	-C panic=abort \
	-C link-arg=-fwasm-exceptions \
	-C link-arg=-sSIDE_MODULE=2 \
	-C link-arg=-lSDL2 \
	-C link-arg=-lhtml5
CFLAGS += $(WASM_PREFIX_MAP_FLAGS)
CXXFLAGS += $(WASM_PREFIX_MAP_FLAGS)
endif

CARGO_OPTS := --release --target $(TARGET) -Zbuild-std=std,panic_abort

ifneq (,$(or $(findstring windows,$(TARGET)),$(findstring darwin,$(TARGET))))
CARGO_OPTS += --features sdl2_static
else
CARGO_OPTS += --features sdl2_dynamic
endif

# Tool options
CLIPPY_OPTS := -q -- --no-deps
MATURIN_OPTS := --manylinux off

# PyO3 environment
ifneq ($(TARGET),$(WASM_TARGET))
PYTHON ?= python3
PYO3_PYTHON ?= $(shell which $(PYTHON))
PYO3_ENVIRONMENT_SIGNATURE ?= $(shell $(PYTHON) -c \
	"import sys,platform; v=sys.version_info; \
	a=platform.architecture()[0]; \
	print(f'{sys.implementation.name}-{v.major}.{v.minor}-{a}')")

lint build test-unit test: export PYO3_PYTHON := $(PYO3_PYTHON)
lint build test-unit test: export PYO3_ENVIRONMENT_SIGNATURE := $(PYO3_ENVIRONMENT_SIGNATURE)
endif

.PHONY: \
	all clean distclean update format lint build install test-unit test \
	clean-wasm lint-wasm build-wasm start-test-server test-wasm \
	pages

all: build

clean:
	@cd $(CRATES_DIR); cargo clean --target $(TARGET)

distclean:
	@rm -rf $(DIST_DIR)
	@rm -rf $(CRATES_DIR)/target

update:
	@rustup -q update
	@cargo -q install cargo-outdated
	@cd $(CRATES_DIR); cargo -q update
	@cd $(CRATES_DIR); cargo -q outdated --root-deps-only
	@pip3 install --upgrade pip
	@pip3 -q install -U -r $(PYTHON_DIR)/requirements.txt

format:
	@cd $(CRATES_DIR); cargo fmt -- --emit=files
	@ruff format $(ROOT_DIR)
	@npx prettier --write --log-level warn "$(ROOT_DIR)/**/*.{css,html,js,json}"

lint:
	@cd $(CRATES_DIR); cargo clippy $(CARGO_OPTS) $(CLIPPY_OPTS)
	@ruff check $(ROOT_DIR)

build:
	@rustup component add rust-src
	@rustup target add $(TARGET)
	@$(SCRIPTS_DIR)/generate_pyi_docstrings
	@$(SCRIPTS_DIR)/generate_docs
	@cp LICENSE $(PYTHON_DIR)/pyxel
	@cd $(PYTHON_DIR); \
		RUSTFLAGS="$(RUSTFLAGS)" \
		CFLAGS="$(CFLAGS)" \
		CXXFLAGS="$(CXXFLAGS)" \
		maturin build -o ../$(DIST_DIR) $(CARGO_OPTS) $(MATURIN_OPTS)

install: build
	@pip3 install --force-reinstall "$$(ls -rt $(DIST_DIR)/*.whl | tail -n 1)"

test-unit: install
	@cd $(ROOT_DIR); python -m pytest python/tests/ -v

test: test-unit
	@CARGO_OPTS="$(CARGO_OPTS)" $(SCRIPTS_DIR)/run_tests

clean-wasm:
	@$(MAKE) clean TARGET=$(WASM_TARGET)

lint-wasm:
	@$(MAKE) lint TARGET=$(WASM_TARGET)

build-wasm:
	@embuilder build sdl2 --pic
	@rm -f $(DIST_DIR)/*-emscripten_*.whl
	@$(MAKE) build TARGET=$(WASM_TARGET)
	@$(SCRIPTS_DIR)/check_wasm_wheel
	@$(SCRIPTS_DIR)/install_wasm_wheel

start-test-server:
	@$(SCRIPTS_DIR)/start_test_server

test-wasm: build-wasm start-test-server

pages:
	@cd $(ROOT_DIR)/web && npx @tailwindcss/cli -i styles/input.css -o styles.css --minify
