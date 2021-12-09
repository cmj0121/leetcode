SRC := $(wildcard problems/*/*.rs)
BIN := $(subst .rs,,$(SRC))

.PHONY: all clean test run build upgrade help

all: $(BIN)		# default action
	@pre-commit install --install-hooks > /dev/null
	@git config commit.template .git-commit-template

clean:			# clean-up environment
	@rm -f $(BIN)

test:			# run test

run:			# run in the local environment

upgrade:		# upgrade all the necessary packages
	pre-commit autoupdate

help:			# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

%: %.rs
	rustc $< -o $@
