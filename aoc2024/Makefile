# Copyright 2024 Ian Lewis
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

SHELL := /bin/bash
REPO_ROOT = $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
REPO_NAME = $(shell basename "$(REPO_ROOT)")

# My personal Advent of Code solutions repository is a private repo that
# contains my personal solutions to the Advent of Code challenges and is used
# for integration test purposes.
SOLUTIONS_REPO ?= ../../advent-of-code-solutions

# The help command prints targets in groups. Help documentation in the Makefile
# uses comments with double hash marks (##). Documentation is printed by the
# help target in the order in appears in the Makefile.
#
# Make targets can be documented with double hash marks as follows:
#
#	target-name: ## target documentation.
#
# Groups can be added with the following style:
#
#	## Group name

.PHONY: help
help: ## Print all Makefile targets (this message).
	@echo "$(REPO_NAME) Makefile"
	@echo "Usage: make [COMMAND]"
	@echo ""
	@set -euo pipefail; \
		normal=""; \
		cyan=""; \
		if [ -t 1 ]; then \
			normal=$$(tput sgr0); \
			cyan=$$(tput setaf 6); \
		fi; \
		grep --no-filename -E '^([/a-z.A-Z0-9_%-]+:.*?|)##' $(MAKEFILE_LIST) | \
			awk \
				--assign=normal="$${normal}" \
				--assign=cyan="$${cyan}" \
				'BEGIN {FS = "(:.*?|)## ?"}; { \
					if (length($$1) > 0) { \
						printf("  " cyan "%-25s" normal " %s\n", $$1, $$2); \
					} else { \
						if (length($$2) > 0) { \
							printf("%s\n", $$2); \
						} \
					} \
				}'

package-lock.json:
	npm install

node_modules/.installed: package.json package-lock.json
	npm ci
	touch node_modules/.installed

.venv/bin/activate:
	python -m venv .venv

.venv/.installed: .venv/bin/activate requirements.txt
	./.venv/bin/pip install -r requirements.txt --require-hashes
	touch .venv/.installed

## Linters
#####################################################################

lint: clippy ## Run all linters.

.PHONY: clippy
clippy: ## Runs clippy linter.
	@set -euo pipefail; \
		exit_code=0; \
		files=$$( \
			git ls-files --deduplicate \
				'Cargo.toml' '*/Cargo.toml' \
				| while IFS='' read -r f; do [ -f "$${f}" ] && echo "$${f}" || true; done \
		); \
		for f in $${files}; do\
			echo "Running clippy on $${f}"; \
			if ! cargo clippy --manifest-path "$${f}" -- -D warnings; then \
				exit_code=1; \
			fi; \
		done; \
		exit "$${exit_code}"

## Solutions
#####################################################################

DAYS = $(wildcard day*)

all: $(DAYS) ## Build all solutions

define GEN_RULE
.PHONY: $1
$1: ## Make each day's solution binary.
	cargo build --manifest-path "$1/Cargo.toml"
endef

$(foreach var,$(DAYS),$(eval $(call GEN_RULE,$(var))))

## Tests
#####################################################################

unit-tests: ## Run unit tests.
	@set -euo pipefail;\
		for i in day*; do\
			cargo test --manifest-path "$$i/Cargo.toml";\
		done

integration-tests: ## Run integration tests.
	@set -euo pipefail;\
		for i in day*; do\
			cargo build --manifest-path "$${i}/Cargo.toml";\
			SOLUTION=$$("$${i}/target/debug/$${i}" < "$(SOLUTIONS_REPO)/2024/$${i}/input.txt");\
			EXPECTED="$$(cat "$(SOLUTIONS_REPO)/2024/$${i}/solution.txt")";\
			[ "$${SOLUTION}" == "$${EXPECTED}" ] || (echo "$${i} failed!" && exit 1);\
		done;


## Maintenance
#####################################################################

clean: ## Clean up build files.
	@set -euo pipefail;\
		for i in day*; do\
			rm -rf "$$i/target";\
		done;
