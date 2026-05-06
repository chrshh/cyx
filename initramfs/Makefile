.PHONY: all clean rootfs image

# Use bash with pipefail so failures inside `find | cpio | gzip` actually
# fail the recipe instead of silently producing an empty archive.
SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c

CC = gcc
CFLAGS = -Wall -Wextra -Wpedantic -O2 $(EXTRA_FLAGS)

ROOTFS := rootfs
CJSH   := ../build/cjsh
IMAGE  := initramfs.cpio.gz

all: init

init: cinit.c
	$(CC) $(CFLAGS) -static -o init cinit.c

# Stage the rootfs tree: binaries + empty mount points.
# /etc/{passwd,group,hostname} live in the repo, so they're already in $(ROOTFS).
rootfs: $(ROOTFS)/init $(ROOTFS)/bin/cjsh
	mkdir -p $(ROOTFS)/dev $(ROOTFS)/proc $(ROOTFS)/sys $(ROOTFS)/tmp $(ROOTFS)/root

$(ROOTFS)/init: init
	install -m 755 init $(ROOTFS)/init

$(ROOTFS)/bin/cjsh: $(CJSH)
	install -D -m 755 $(CJSH) $(ROOTFS)/bin/cjsh

# Pack the staged tree into a newc-format cpio.gz the kernel can load as initrd.
image: rootfs
	cd $(ROOTFS) && find . -print0 | cpio --null --create --format=newc --quiet | gzip -9 > ../$(IMAGE)

clean:
	rm -f init $(IMAGE)
	rm -rf $(ROOTFS)/init $(ROOTFS)/bin $(ROOTFS)/dev $(ROOTFS)/proc $(ROOTFS)/sys $(ROOTFS)/tmp $(ROOTFS)/root
