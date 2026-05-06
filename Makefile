.PHONY: all clean init cjsh build-image image run run-shell-only run-graphical run-graphical-shell-only run-g run-gs

ROOT      := $(CURDIR)
BUILD     := $(ROOT)/build
USERLAND  := $(ROOT)/userland
INITRAMFS := $(ROOT)/initramfs

DOCKER_IMAGE := cjyx-static

# Borrow the host kernel for now. Swap this to your own bzImage later.
KERNEL := /usr/lib/modules/$(shell uname -r)/vmlinuz
CPIO   := $(INITRAMFS)/initramfs.cpio.gz

# -enable-kvm if /dev/kvm is accessible, else fall back to TCG.
KVM := $(shell test -r /dev/kvm && test -w /dev/kvm && echo -enable-kvm)

QEMU_COMMON := qemu-system-x86_64 \
  -kernel $(KERNEL) \
  -initrd $(CPIO) \
  -m 256 \
  $(KVM)

# Headless: serial-only output piped into your terminal (Ghostty renders it).
QEMU_HEADLESS := $(QEMU_COMMON) -nographic

# Graphical: GTK window, kernel renders to virtual VGA / framebuffer console.
QEMU_GRAPHICAL := $(QEMU_COMMON) -display gtk

all: image

$(BUILD):
	mkdir -p $(BUILD)

# init: built statically on the host (only depends on libc.a, which Arch ships).
init: $(BUILD)/init

$(BUILD)/init: $(INITRAMFS)/cinit.c | $(BUILD)
	$(MAKE) -C $(INITRAMFS) init
	cp $(INITRAMFS)/init $(BUILD)/init

# cjsh: built statically inside Docker (host doesn't have libreadline.a).
cjsh: $(BUILD)/cjsh

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

# image: stage rootfs and pack initramfs.cpio.gz
image: $(BUILD)/init $(BUILD)/cjsh
	$(MAKE) -C $(INITRAMFS) image

# Boot the full chain: kernel -> /init (cinit) -> /bin/cjsh
run: image
	$(QEMU_HEADLESS) -append "console=ttyS0"

# Bypass cinit and exec cjsh directly as PID 1, for isolating shell vs. init bugs.
run-shell-only: image
	$(QEMU_HEADLESS) -append "console=ttyS0 rdinit=/bin/cjsh"

# Graphical: opens an SDL window, kernel uses tty0/fbcon. Closer to what the
# Pi will look like over HDMI than the Ghostty-rendered serial view.
run-graphical: image
	$(QEMU_GRAPHICAL)

run-graphical-shell-only: image
	$(QEMU_GRAPHICAL) -append "rdinit=/bin/cjsh"

# Short aliases.
run-g:  run-graphical
run-gs: run-graphical-shell-only

clean:
	$(MAKE) -C $(USERLAND) clean
	$(MAKE) -C $(INITRAMFS) clean
	rm -rf $(BUILD)
