.PHONY: all clean test build upgrade help

all: build		# default action
	@pre-commit install --install-hooks > /dev/null
	@git config commit.template .git-commit-template

clean:			# clean-up environment
	@rm -f $(BIN)

test:			# run test
	cargo test --verbose --all

build: test		# build the project
	cargo build --all-targets

upgrade:		# upgrade all the necessary packages
	pre-commit autoupdate

help:			# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

%: %.rs
	rustc $< -o $@
