#
# Prerequisites:
#   - git, make, cmake, rustup, python 3.8+, node.js
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
#   - Test: make clean test (includes watch)
#
# WASM (pyxel-pocket):
#   - Setup once: install Emscripten SDK
#   - Each new shell: source emsdk/emsdk_env.sh
#   - Lint: make lint-wasm
#   - Build: make clean-wasm build-wasm
#   - Test: make test-wasm (builds and starts local server at :8000)
#
# Web pages:
#   - Setup once: cd web && npm install
#   - Build: make pages
#

# Project directories
ROOT_DIR = .
DIST_DIR = $(ROOT_DIR)/dist
CRATES_DIR = $(ROOT_DIR)/crates
PYTHON_DIR = $(ROOT_DIR)/python
EXAMPLES_DIR = $(PYTHON_DIR)/pyxel/examples
SCRIPTS_DIR = $(ROOT_DIR)/scripts

# Build targets
TARGET ?= $(shell rustc -vV | awk '/^host:/ {print $$2}')
WASM_TARGET = wasm32-unknown-emscripten

# WASM path remap flags
REMAP_SRC_PATH = $(abspath $(ROOT_DIR))
REMAP_USER_HOME ?= /user
RUST_REMAP_FLAGS = --remap-path-prefix=$(REMAP_SRC_PATH)=/src/pyxel
WASM_PREFIX_MAP_FLAGS = -ffile-prefix-map=$(REMAP_SRC_PATH)=/src/pyxel
ifneq ($(HOME),)
RUST_REMAP_FLAGS += --remap-path-prefix=$(HOME)=$(REMAP_USER_HOME)
WASM_PREFIX_MAP_FLAGS += -ffile-prefix-map=$(HOME)=$(REMAP_USER_HOME)
endif

# Build options
WASM_RUSTFLAGS = \
	$(RUST_REMAP_FLAGS) \
	-C panic=abort \
	-C link-arg=-fwasm-exceptions \
	-C link-arg=-sUSE_SDL=2 \
	-C link-arg=-sALLOW_MEMORY_GROWTH

CARGO_OPTS = --release --target $(TARGET) -Zbuild-std=std,panic_abort

ifneq (,$(or $(findstring windows,$(TARGET)),$(findstring darwin,$(TARGET))))
CARGO_OPTS += --features sdl2_static
else
CARGO_OPTS += --features sdl2_dynamic
endif

# Tool options
CLIPPY_OPTS = -q -- --no-deps
MATURIN_OPTS = --manylinux off

# PyO3 environment
ifneq ($(TARGET),$(WASM_TARGET))
PYTHON ?= python3
PYO3_PYTHON ?= $(shell which $(PYTHON))
PYO3_ENVIRONMENT_SIGNATURE ?= $(shell $(PYTHON) -c \
	"import sys,platform; v=sys.version_info; \
	a=platform.architecture()[0]; \
	print(f'{sys.implementation.name}-{v.major}.{v.minor}-{a}')")

lint build test: export PYO3_PYTHON := $(PYO3_PYTHON)
lint build test: export PYO3_ENVIRONMENT_SIGNATURE := $(PYO3_ENVIRONMENT_SIGNATURE)
endif

WASM_DIST_DIR = $(DIST_DIR)/wasm

.PHONY: \
	all clean distclean update format lint build install test test-pocket \
	clean-wasm lint-wasm build-wasm test-wasm \
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

lint:
	@cd $(CRATES_DIR); cargo clippy $(CARGO_OPTS) $(CLIPPY_OPTS)
	@ruff check $(ROOT_DIR)

build: format
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

test: install
	@CARGO_OPTS="$(CARGO_OPTS)" $(SCRIPTS_DIR)/run_tests

test-pocket:
	@CARGO_OPTS="$(CARGO_OPTS)" $(SCRIPTS_DIR)/run_pocket_tests

clean-wasm:
	@cd $(CRATES_DIR); cargo clean -p pyxel-pocket --target $(WASM_TARGET)

lint-wasm:
	@cd $(CRATES_DIR); cargo clippy -p pyxel-pocket \
		--release --target $(WASM_TARGET) -Zbuild-std=std,panic_abort \
		--features sdl2_dynamic $(CLIPPY_OPTS) || true

build-wasm:
	@cd $(CRATES_DIR); \
		RUSTFLAGS="$(WASM_RUSTFLAGS)" \
		CFLAGS="$(WASM_PREFIX_MAP_FLAGS)" \
		CXXFLAGS="$(WASM_PREFIX_MAP_FLAGS)" \
		cargo build -p pyxel-pocket --bin pyxel-pocket \
		--release --target $(WASM_TARGET) -Zbuild-std=std,panic_abort \
		--features sdl2_dynamic
	@cp $(CRATES_DIR)/target/$(WASM_TARGET)/release/deps/pyxel_pocket.wasm $(ROOT_DIR)/wasm/
	@cp $(CRATES_DIR)/target/$(WASM_TARGET)/release/deps/pyxel_pocket.js $(ROOT_DIR)/wasm/

test-wasm: build-wasm
	@$(SCRIPTS_DIR)/start_test_server

pages:
	@cd $(ROOT_DIR)/web && npx @tailwindcss/cli -i styles/input.css -o styles.css --minify
