#
# Makefile for anion documentation
#

ANION_SRC ?= "../anion"

all:
	pushd ${ANION_SRC} && \
	cargo doc && \
	popd
	cp -r ${ANION_SRC}/target/doc/ .

