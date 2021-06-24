.PHONY: all clean help

SRC := $(wildcard */*.rs)
BIN := $(subst .rs,,$(SRC))

all: $(BIN)	# build all sub-folder

clean:		# clean all

help:		# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

%: %.rs
	rustc $< -o $@
