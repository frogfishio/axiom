.PHONY: bump dist clean

VERSION_FILE := VERSION
BUILD_FILE := BUILD
BIN_NAME := sda
DIST_ROOT := dist/bin
TARGET_DIR := target
DIST_TRIPLE := $(shell uname -s | tr '[:upper:]' '[:lower:]')-$(shell uname -m | tr '[:upper:]' '[:lower:]')
DIST_DIR := $(DIST_ROOT)/$(DIST_TRIPLE)
RELEASE_BIN := $(TARGET_DIR)/release/$(BIN_NAME)

bump:
	@set -eu; \
	current="$$(tr -d '[:space:]' < $(VERSION_FILE))"; \
	next="$$(printf '%s\n' "$$current" | awk -F. '
		NF == 1 && $$1 ~ /^[0-9]+$$/ { $$1 += 1; print $$1; next }
		NF >= 2 {
			for (i = 1; i <= NF; i++) {
				if ($$i !~ /^[0-9]+$$/) {
					exit 1;
				}
			}
			$$NF += 1;
			for (i = 1; i <= NF; i++) {
				printf "%s%s", $$i, (i < NF ? "." : ORS)
			}
			next
		}
		exit 1
	')" || { echo "Error: unsupported VERSION format in $(VERSION_FILE)" >&2; exit 1; }; \
	printf '%s\n' "$$next" > $(VERSION_FILE); \
	echo "Bumped $(VERSION_FILE): $$current -> $$next"

dist:
	@set -eu; \
	current_build="$$(tr -d '[:space:]' < $(BUILD_FILE))"; \
	case "$$current_build" in \
		""|*[!0-9]*) echo "Error: unsupported BUILD format in $(BUILD_FILE)" >&2; exit 1 ;; \
		*) next_build="$$((current_build + 1))" ;; \
	esac; \
	printf '%s\n' "$$next_build" > $(BUILD_FILE); \
	echo "Bumped $(BUILD_FILE): $$current_build -> $$next_build"; \
	cargo build --release -p sda-cli --bin $(BIN_NAME); \
	mkdir -p $(DIST_DIR); \
	cp $(RELEASE_BIN) $(DIST_DIR)/$(BIN_NAME); \
	echo "Copied $(RELEASE_BIN) -> $(DIST_DIR)/$(BIN_NAME)"

clean:
	@rm -rf $(TARGET_DIR)
	@echo "Removed $(TARGET_DIR)"