SHELL := /bin/sh

.POSIX:
.PHONY: all clean doc rust rust_release

all: rust doc

rust:
	+${MAKE} all -C rust

rust_release:
	+${MAKE} release -C rust

clean:
	+${MAKE} clean -C rust
	+${MAKE} clean -C docs/diagrams

doc:
	+${MAKE} all -C docs/diagrams
	+${MAKE} doc -C rust
