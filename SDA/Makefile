# SDA POC smoke runner
#
# Usage:
#   make -C poc/SDA flex

SHELL := /bin/bash

THIS_MAKEFILE := $(abspath $(lastword $(MAKEFILE_LIST)))
THIS_DIR := $(dir $(THIS_MAKEFILE))
ROOT := $(abspath $(THIS_DIR)/../..)
OUT_DIR := $(ROOT)/out/tmp

SEED_BIN := $(ROOT)/bin/seed
CST_BIN ?= $(ROOT)/src/1_cst/cst
PACKRUN_BIN ?= $(ROOT)/src/2_packrun/packrun

SDA_SPEC := $(THIS_DIR)/sda.grit
SDA_SRC := $(THIS_DIR)/smoke.sda
SDA_SEED_PACK := $(OUT_DIR)/sda.seed.pack.json

.PHONY: flex clean

flex:
	@set -eu; \
	mkdir -p "$(OUT_DIR)"; \
	if [ ! -x "$(CST_BIN)" ] || [ ! -x "$(PACKRUN_BIN)" ]; then \
		echo "[sda] missing $(CST_BIN) or $(PACKRUN_BIN)" 1>&2; \
		echo "[sda] build with: make -C src/lib out && make -C src/1_cst && make -C src/2_packrun" 1>&2; \
		exit 2; \
	fi; \
	echo "[sda] seed -> $(SDA_SEED_PACK)"; \
	"$(SEED_BIN)" --emit-pack-json "$(SDA_SPEC)" > "$(SDA_SEED_PACK)"; \
	test -s "$(SDA_SEED_PACK)"; \
	echo "[sda] cst (flex)"; \
	GRIT_FORCE_FLEX=1 "$(CST_BIN)" --pack "$(SDA_SEED_PACK)" --source "$(SDA_SRC)" --out "$(OUT_DIR)/sda.flex.cst.json"; \
	echo "[sda] packrun (flex)"; \
	GRIT_FORCE_FLEX=1 "$(PACKRUN_BIN)" --pack "$(SDA_SEED_PACK)" --source "$(SDA_SRC)" --out "$(OUT_DIR)/sda.flex.parsetree.json"; \
	echo "[sda] OK"

clean:
	rm -f "$(SDA_SEED_PACK)" "$(OUT_DIR)/sda.flex.cst.json" "$(OUT_DIR)/sda.flex.parsetree.json"
