#
# Makefile for anion documentation
#

ANION_SRC ?= ../anion

DOC_DIR = docs/

.PHONY: all update-git

all: ${DOC_DIR}
	pushd ${ANION_SRC} && cargo doc && popd
	cp -r ${ANION_SRC}/target/doc/ ${DOC_DIR}

update-git: all
	git status

${DOC_DIR}:
	mkdir -p $@

