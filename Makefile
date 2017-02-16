#
# Makefile for anion documentation
#

ANION_SRC ?= ../anion
DOC_DIR = ./docs/

CP_DIR = rsync -av --exclude='*grammar.peg.rs.html' --delete


.PHONY: all update-git

all: ${DOC_DIR}
	pushd ${ANION_SRC} && cargo doc --lib --no-deps; popd
	${CP_DIR} ${ANION_SRC}/target/doc/ ${DOC_DIR}

update-git: all
	git status

${DOC_DIR}:
	mkdir -p $@

