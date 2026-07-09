.PHONY: all clean rootfs image stage-cmd stage-cmd-rs size

SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c

# init is the guest PID 1 — it must be an aarch64 binary. Cross-compile it on
# the host with the aarch64-linux-gnu toolchain (override CROSS_COMPILE to match
# whatever prefix your distro ships). Static so it has no runtime lib deps.
CROSS_COMPILE ?= aarch64-linux-gnu-
CC = $(CROSS_COMPILE)gcc
CFLAGS = -Wall -Wextra -Wpedantic -O2 $(EXTRA_FLAGS)

ROOTFS  := rootfs
DEBIAN  := ../build/debian-rootfs
# Marker touched by the top-level Makefile every time the Debian rootfs is
# re-exported. .staged depends on it so a refreshed export (e.g. an arch change)
# forces a full re-extract instead of layering new binaries onto a stale base.
DEBIAN_MARKER := ../build/.debian_rootfs
CJSH    := ../build/cjsh
DISPLAY := ../build/display
SQUISHY := ../build/squishy
DINKY   := ../build/dinky
CMDDIR  := ../build/cmd
CMDRSDIR := ../build/cmd-rs
IMAGE   := ../build/disk.img
# 12G: Mesa (llvmpipe + v3d/vc4) + Xwayland + fonts + Debian baseline, plus
# ext4 overhead and headroom for /home growth + apps you'll write. GTK/Qt/foot
# were dropped, so the extracted rootfs is well under the old ~2.5GB.
SIZE    := 12G

# The list of Debian core utilities to assassinate. 
# Add or remove names from this list as you code your own replacements.
PURGE_LIST := ls cat mkdir rm cp mv echo pwd date true false chown chmod uname whoami clear sleep

all: init

init: cinit.c
	$(CC) $(CFLAGS) -static -o init cinit.c

# Layered staging: Debian userland goes down first as the base, then our own
# binaries and config files overlay on top. 
rootfs: $(ROOTFS)/.staged $(ROOTFS)/init $(ROOTFS)/bin/cjsh $(ROOTFS)/bin/display $(ROOTFS)/bin/squishy $(ROOTFS)/bin/dinky stage-cmd stage-cmd-rs $(ROOTFS)/etc/passwd
	mkdir -p $(ROOTFS)/dev $(ROOTFS)/proc $(ROOTFS)/sys $(ROOTFS)/tmp $(ROOTFS)/root $(ROOTFS)/home $(ROOTFS)/run

# Re-extract whenever the Debian export is refreshed (marker is newer),
# otherwise the staged base is reused as-is.
$(ROOTFS)/.staged: $(DEBIAN_MARKER)
	rm -rf $(ROOTFS)
	mkdir -p $(ROOTFS)
	cp -a $(DEBIAN)/. $(ROOTFS)/
	# --- THE BLOAT PURGE ---
	# 1. Delete Debian's shells
	rm -f $(ROOTFS)/bin/bash $(ROOTFS)/usr/bin/bash
	rm -f $(ROOTFS)/bin/dash $(ROOTFS)/usr/bin/dash
	# 2. Force the standard POSIX shell path to point to your custom shell
	rm -f $(ROOTFS)/bin/sh
	ln -sf cjsh $(ROOTFS)/bin/sh
	# 3. Purge Debian Coreutils
	for cmd in $(PURGE_LIST); do \
		rm -f $(ROOTFS)/bin/$$cmd $(ROOTFS)/usr/bin/$$cmd; \
	done
# --- BULK DIRECTORY PURGE ---
	# 4. Nuke all Debian documentation, manuals, and info pages
	rm -rf $(ROOTFS)/usr/share/doc $(ROOTFS)/usr/share/man $(ROOTFS)/usr/share/info
	# 5. Nuke package manager metadata and caches 
	rm -rf $(ROOTFS)/var/lib/apt/lists/* $(ROOTFS)/var/cache/apt/*
	rm -rf $(ROOTFS)/var/lib/dpkg/info/*
# 6. Nuke foreign language translations
	rm -rf $(ROOTFS)/usr/share/locale
	touch $@

# All overlay targets MUST depend on .staged so Make knows to re-copy them
# if the Debian base ever gets wiped.
$(ROOTFS)/init: init $(ROOTFS)/.staged
	install -D -m 755 init $(ROOTFS)/init

$(ROOTFS)/bin/cjsh: $(CJSH) $(ROOTFS)/.staged
	install -D -m 755 $(CJSH) $(ROOTFS)/bin/cjsh

$(ROOTFS)/bin/display: $(DISPLAY) $(ROOTFS)/.staged
	install -D -m 755 $(DISPLAY) $(ROOTFS)/bin/display

$(ROOTFS)/bin/squishy: $(SQUISHY) $(ROOTFS)/.staged
	install -D -m 755 $(SQUISHY) $(ROOTFS)/bin/squishy

# asemics is a Rust workspace member — installed by stage-cmd-rs below (which
# also creates the `ase` alias), not by a dedicated rule.

$(ROOTFS)/bin/dinky: $(DINKY) $(ROOTFS)/.staged
	install -D -m 755 $(DINKY) $(ROOTFS)/bin/dinky

# palmer is a Rust workspace member — installed by stage-cmd-rs, not a rule here.

$(ROOTFS)/etc/passwd: etc/passwd etc/group etc/hostname $(ROOTFS)/.staged
	mkdir -p $(ROOTFS)/etc
	install -m 644 etc/passwd $(ROOTFS)/etc/passwd
	install -m 644 etc/group $(ROOTFS)/etc/group
	install -m 644 etc/hostname $(ROOTFS)/etc/hostname

stage-cmd: $(ROOTFS)/.staged
	mkdir -p $(ROOTFS)/bin
	if [ -d $(CMDDIR) ]; then \
		for f in $(CMDDIR)/*; do \
			[ -f "$$f" ] && [ -x "$$f" ] && install -m 755 "$$f" $(ROOTFS)/bin/; \
		done; \
	fi

# Same pattern as stage-cmd but for the Rust workspace output. Iterates
# build/cmd-rs/ and installs every executable file into rootfs/bin. Safe to
# run when the directory is empty (no Rust commands written yet).
stage-cmd-rs: $(ROOTFS)/.staged
	mkdir -p $(ROOTFS)/bin
	if [ -d $(CMDRSDIR) ]; then \
		for f in $(CMDRSDIR)/*; do \
			[ -f "$$f" ] && [ -x "$$f" ] && install -m 755 "$$f" $(ROOTFS)/bin/; \
		done; \
		if [ -x $(ROOTFS)/bin/asemics ]; then ln -sf asemics $(ROOTFS)/bin/ase; fi; \
	fi

image: rootfs
	mkdir -p ../build
	truncate -s $(SIZE) $(IMAGE)
	# -O ^metadata_csum: mkfs's directory pre-population (-d) doesn't reserve
	# room for per-directory-leaf checksums, so with metadata_csum the guest
	# kernel throws "No space for directory leaf checksum" and errors the fs on
	# first read. Disabling it sidesteps the whole class (fine for a rebuilt
	# appliance image).
	mkfs.ext4 -F -q -O ^metadata_csum -d $(ROOTFS) $(IMAGE)

clean:
	rm -f init $(IMAGE)
	rm -rf $(ROOTFS)

size:
	@echo "=== Total OS Size ==="
	@du -sh $(ROOTFS)
	@echo ""
	@echo "=== Top 10 Largest Folders in OS ==="
	@du -sh $(ROOTFS)/* | sort -rh | head -n 10
