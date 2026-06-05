.PHONY: all clean clean-data clean-all init cjsh display cmd cmd-rs squishy textr dinky build-image image run run-shell-only run-graphical run-graphical-shell-only run-g run-gs
ROOT      := $(CURDIR)
BUILD     := $(ROOT)/build
USERLAND  := $(ROOT)/userland
DISK      := $(ROOT)/disk
DOCKER_IMAGE := cjyx-static
SQUISHY_SRC := $(USERLAND)/apps/squishy
DINKY_SRC   := $(USERLAND)/apps/dinky
CMDRS_SRC   := $(USERLAND)/cmd-rs
# Our own kernel — built from kernel/src with virtio_gpu, bochs, ext4, etc all
# compiled in (no module loading infrastructure in this OS).
KERNEL := $(ROOT)/kernel/src/arch/x86/boot/bzImage
IMG    := $(BUILD)/disk.img
DATA   := $(BUILD)/data.img
KVM := $(shell test -r /dev/kvm && test -w /dev/kvm && echo -enable-kvm)
# 2048MB: GTK/Qt apps blow through 256MB instantly. Mesa llvmpipe alone
# wants several hundred MB of working set, plus per-app overhead.
QEMU_COMMON := qemu-system-x86_64 \
  -kernel $(KERNEL) \
  -drive file=$(IMG),format=raw,if=virtio \
  -drive file=$(DATA),format=raw,if=virtio \
  -m 2048 \
  $(KVM)
QEMU_HEADLESS := $(QEMU_COMMON) -nographic
OVMF := /usr/share/edk2/x64/OVMF.4m.fd
# virtio-gpu-pci: emulates a virtio-gpu device that our kernel binds to via
# the (now built-in) virtio_gpu driver. Gives wlroots a real DRM device with
# modesetting, GBM, and atomic — everything simpledrm couldn't provide.
# (virtio-vga is the same family but with legacy VGA compat; we boot via
# UEFI/GOP so we don't need that.)
QEMU_GRAPHICAL := $(QEMU_COMMON) -display gtk -bios $(OVMF) -device virtio-gpu-pci
ROOT_CMDLINE := root=/dev/vda rootfstype=ext4 rw

all: image

$(BUILD):
	mkdir -p $(BUILD)

$(DATA): | $(BUILD)
	truncate -s 256M $(DATA)
	mkfs.ext4 -F -q $(DATA)

init: $(BUILD)/init
$(BUILD)/init: $(DISK)/cinit.c | $(BUILD)
	$(MAKE) -C $(DISK) init
	cp $(DISK)/init $(BUILD)/init

cjsh: $(BUILD)/cjsh
display: $(BUILD)/display
squishy: $(BUILD)/squishy
textr: $(BUILD)/textr
dinky: $(BUILD)/dinky

build-image:
	docker build \
	  --build-arg BUILD_TARGET=static \
	  -t $(DOCKER_IMAGE) \
	  -f $(USERLAND)/Dockerfile \
	  $(USERLAND)

$(BUILD)/cjsh: build-image | $(BUILD)
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker cp $$cid:/cjyx/cjsh/cjsh $(BUILD)/cjsh; \
	  docker rm $$cid >/dev/null

$(BUILD)/display: build-image | $(BUILD)
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker cp $$cid:/cjyx/display/display $(BUILD)/display; \
	  docker rm $$cid >/dev/null

$(BUILD)/textr: build-image | $(BUILD)
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker cp $$cid:/cjyx/apps/textr/textr $(BUILD)/textr; \
	  docker rm $$cid >/dev/null

# Squishy is Rust, so it builds in its own toolchain image (rust:1-bookworm)
# instead of the cjyx-static C image. The Python helper handles the docker
# build/run dance and the binary install.
$(BUILD)/squishy: $(SQUISHY_SRC)/build.py $(SQUISHY_SRC)/Dockerfile.build $(SQUISHY_SRC)/Cargo.toml $(SQUISHY_SRC)/Cargo.lock $(wildcard $(SQUISHY_SRC)/src/*.rs) $(wildcard $(SQUISHY_SRC)/assets/*) | $(BUILD)
	python3 $(SQUISHY_SRC)/build.py $(BUILD)/squishy

# Same Rust-in-docker pattern as squishy. Dinky's builder image is leaner
# (no xkbcommon-dev — ratatui + crossterm are pure Rust).
$(BUILD)/dinky: $(DINKY_SRC)/build.py $(DINKY_SRC)/Dockerfile.build $(DINKY_SRC)/Cargo.toml $(DINKY_SRC)/Cargo.lock $(wildcard $(DINKY_SRC)/src/*.rs) | $(BUILD)
	python3 $(DINKY_SRC)/build.py $(BUILD)/dinky

# Full Debian userland: docker export gives us a tarball of the entire image
# filesystem, which includes Mesa drivers, GTK/Qt libs, foot terminal,
# Xwayland, dbus-daemon, seatd, etc. — everything a real userland needs.
# We exclude /cjyx (we ship our own builds via dedicated targets) and the
# kernel-mounted virtual filesystems (/proc /sys /dev mountpoints get
# created fresh in the rootfs and mounted at runtime).
$(BUILD)/.debian_rootfs: build-image | $(BUILD)
	rm -rf $(BUILD)/debian-rootfs
	mkdir -p $(BUILD)/debian-rootfs
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker export $$cid | tar -x -C $(BUILD)/debian-rootfs \
	    --exclude='cjyx' \
	    --exclude='proc/*' \
	    --exclude='sys/*' \
	    --exclude='dev/*' \
	    --exclude='.dockerenv' ; \
	  docker rm $$cid >/dev/null
	touch $@

cmd: build-image | $(BUILD)
	rm -rf $(BUILD)/cmd
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker cp $$cid:/cjyx/cmd/bin $(BUILD)/cmd; \
	  docker rm $$cid >/dev/null

# Rust workspace of coreutil-style commands. Same Rust-in-docker pattern as
# squishy/dinky but produces N binaries (one per workspace member crate)
# into build/cmd-rs/. The disk Makefile's stage-cmd-rs target installs each
# one into rootfs/bin alongside the C commands from build/cmd/.
cmd-rs: $(CMDRS_SRC)/build.py $(CMDRS_SRC)/Dockerfile.build $(CMDRS_SRC)/Cargo.toml $(wildcard $(CMDRS_SRC)/*/Cargo.toml) $(wildcard $(CMDRS_SRC)/*/src/*.rs) | $(BUILD)
	rm -rf $(BUILD)/cmd-rs
	mkdir -p $(BUILD)/cmd-rs
	python3 $(CMDRS_SRC)/build.py $(BUILD)/cmd-rs

image: $(BUILD)/init $(BUILD)/cjsh $(BUILD)/display $(BUILD)/squishy $(BUILD)/textr $(BUILD)/dinky cmd cmd-rs $(BUILD)/.debian_rootfs $(DATA)
	$(MAKE) -C $(DISK) image

run: image
	$(QEMU_HEADLESS) -append "$(ROOT_CMDLINE) console=ttyS0 init=/init"
run-shell-only: image
	$(QEMU_HEADLESS) -append "$(ROOT_CMDLINE) console=ttyS0 init=/bin/cjsh"
run-graphical: image
	$(QEMU_GRAPHICAL) -append "$(ROOT_CMDLINE) init=/init"
run-graphical-shell-only: image
	$(QEMU_GRAPHICAL) -append "$(ROOT_CMDLINE) init=/bin/cjsh"

run-g:  run-graphical
run-gs: run-graphical-shell-only

clean:
	$(MAKE) -C $(USERLAND) clean
	$(MAKE) -C $(DISK) clean
	rm -rf $(BUILD)/init $(BUILD)/cjsh $(BUILD)/display $(BUILD)/squishy $(BUILD)/textr $(BUILD)/dinky $(BUILD)/cmd $(BUILD)/cmd-rs $(BUILD)/cmd-rs-target $(BUILD)/disk.img

clean-data:
	rm -f $(DATA)
clean-all: clean clean-data
	rm -rf $(BUILD)
