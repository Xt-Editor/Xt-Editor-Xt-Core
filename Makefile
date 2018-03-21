SHELL := /bin/sh

CARGO := cargo
CARGO_OPTS :=

GRAPHVIZ_BUILDER := dot

all: build test doc doc_info

build:
	${CARGO} ${CARGO_OPTS} build

clean:
	${CARGO} ${CARGO_OPTS} clean

ci-check:
	${CARGO} ${CARGO_OPTS} build --verbose --features clippy
	${CARGO} ${CARGO_OPTS} test --verbose --features clippy

test:
	${CARGO} ${CARGO_OPTS} test

bench:
	${CARGO} ${CARGO_OPTS} bench

doc: doc_gen_html doc_gen_info
	${CARGO} ${CARGO_OPTS} doc

doc_gen_html:
	cd doc/sphinx && make html

doc_gen_info:
	cd doc/sphinx && make info

dot_core_relations:
	cd doc/graphviz; \
	${GRAPHVIZ_BUILDER} -Tpng -o core-relations.png \
		core-relations.dot

.PHONY: all build clean ci-check test bench doc doc_gen_html doc_gen_info
