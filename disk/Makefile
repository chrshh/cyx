.PHONY: all clean rootfs image stage-cmd

# pipefail: catch failures inside pipelines (e.g. mkfs barking via tee).
SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c

CC = gcc
CFLAGS = -Wall -Wextra -Wpedantic -O2 $(EXTRA_FLAGS)

ROOTFS := rootfs
CJSH   := ../build/cjsh
CMDDIR := ../build/cmd
IMAGE  := ../build/disk.img
SIZE   := 256M

all: init

init: cinit.c
	$(CC) $(CFLAGS) -static -o init cinit.c

# Stage the rootfs tree: binaries, /etc, and the empty mount points cinit
# expects to bind kernel filesystems onto at boot.
rootfs: $(ROOTFS)/init $(ROOTFS)/bin/cjsh stage-cmd $(ROOTFS)/etc/passwd
	mkdir -p $(ROOTFS)/dev $(ROOTFS)/proc $(ROOTFS)/sys $(ROOTFS)/tmp $(ROOTFS)/root

$(ROOTFS)/init: init
	install -D -m 755 init $(ROOTFS)/init

$(ROOTFS)/bin/cjsh: $(CJSH)
	install -D -m 755 $(CJSH) $(ROOTFS)/bin/cjsh

$(ROOTFS)/etc/passwd: etc/passwd etc/group etc/hostname
	mkdir -p $(ROOTFS)/etc
	install -m 644 etc/passwd $(ROOTFS)/etc/passwd
	install -m 644 etc/group $(ROOTFS)/etc/group
	install -m 644 etc/hostname $(ROOTFS)/etc/hostname

# Drop every static binary from build/cmd/ into rootfs/bin/. .PHONY so make
# always re-runs it — a new file in build/cmd/ wouldn't otherwise trigger
# anything, since no rule names it as a target.
stage-cmd:
	mkdir -p $(ROOTFS)/bin
	if [ -d $(CMDDIR) ]; then \
		for f in $(CMDDIR)/*; do \
			[ -f "$$f" ] && [ -x "$$f" ] && install -m 755 "$$f" $(ROOTFS)/bin/; \
		done; \
	fi

# Build a raw ext4 disk image from the staged tree. mkfs.ext4 -d copies a
# directory's contents into the new filesystem without needing root or a
# loop mount — modern e2fsprogs does it offline. -F forces overwrite of an
# existing image; truncate (re)creates the backing file at $(SIZE) first.
image: rootfs
	mkdir -p ../build
	truncate -s $(SIZE) $(IMAGE)
	mkfs.ext4 -F -q -d $(ROOTFS) $(IMAGE)

clean:
	rm -f init $(IMAGE)
	rm -rf $(ROOTFS)
