#
# Format and lint code:
#   make format
#
# Debug build:
#   make clean build
#
# Debug build and test:
#   make clean test
#
# Release build:
#   make clean build RELEASE=1
#
# Release build and test:
#   make clean test RELEASE=1
#
# Make Python wheel after release build
#   make wheel
#
# Make distribution package after making Python wheel
#   make dist
#

FORWARD_DIR = crates

.PHONY: forward $(MAKECMDGOALS)

forward:
	@$(MAKE) -C $(FORWARD_DIR) $(MAKECMDGOALS)

$(MAKECMDGOALS): forward
