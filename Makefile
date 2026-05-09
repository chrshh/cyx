.PHONY: all clean init cjsh cmd build-image image run run-shell-only run-graphical run-graphical-shell-only run-g run-gs

ROOT      := $(CURDIR)
BUILD     := $(ROOT)/build
USERLAND  := $(ROOT)/userland
DISK      := $(ROOT)/disk

DOCKER_IMAGE := cjyx-static

# Borrow the host kernel for now. Swap this to your own bzImage later.
KERNEL := /usr/lib/modules/$(shell uname -r)/vmlinuz
IMG    := $(BUILD)/disk.img

# -enable-kvm if /dev/kvm is accessible, else fall back to TCG.
KVM := $(shell test -r /dev/kvm && test -w /dev/kvm && echo -enable-kvm)

# virtio-blk is built into the host kernel (CONFIG_VIRTIO_BLK=y), so the
# guest sees the image as /dev/vda without needing module loading.
QEMU_COMMON := qemu-system-x86_64 \
  -kernel $(KERNEL) \
  -drive file=$(IMG),format=raw,if=virtio \
  -m 256 \
  $(KVM)

# Headless: serial-only output piped into your terminal (Ghostty renders it).
QEMU_HEADLESS := $(QEMU_COMMON) -nographic

# UEFI firmware. Required for the EFI framebuffer (efifb), which is the only
# graphics driver compiled into the host kernel. Without UEFI the guest falls
# back to ~720x400 text mode because virtio_gpu/bochs are kernel modules our
# image doesn't carry.
OVMF := /usr/share/edk2/x64/OVMF.4m.fd

# Graphical: GTK window, UEFI boot, kernel uses efifb at firmware-set resolution.
QEMU_GRAPHICAL := $(QEMU_COMMON) -display gtk -bios $(OVMF)

# rw: rootfs must be writable so cinit can mkdir/mount runtime state.
# rootfstype=ext4: skip auto-detect and go straight to the right driver.
ROOT_CMDLINE := root=/dev/vda rootfstype=ext4 rw

all: image

$(BUILD):
	mkdir -p $(BUILD)

# init: built statically on the host (only depends on libc.a, which Arch ships).
init: $(BUILD)/init

$(BUILD)/init: $(DISK)/cinit.c | $(BUILD)
	$(MAKE) -C $(DISK) init
	cp $(DISK)/init $(BUILD)/init

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

# cmd: every static binary built from userland/cmd/*.c, extracted from the
# Docker image into build/cmd/. .PHONY because make can't see new files
# appearing in cmd/ — we always re-extract so newly added commands ship.
cmd: build-image | $(BUILD)
	rm -rf $(BUILD)/cmd
	cid=$$(docker create $(DOCKER_IMAGE)); \
	  docker cp $$cid:/cjyx/cmd/bin $(BUILD)/cmd; \
	  docker rm $$cid >/dev/null

# image: stage rootfs and pack it into a raw ext4 disk image.
image: $(BUILD)/init $(BUILD)/cjsh cmd
	$(MAKE) -C $(DISK) image

# Boot the full chain: kernel -> /init (cinit) -> /bin/cjsh
run: image
	$(QEMU_HEADLESS) -append "$(ROOT_CMDLINE) console=ttyS0 init=/init"

# Bypass cinit and exec cjsh directly as PID 1, for isolating shell vs. init bugs.
run-shell-only: image
	$(QEMU_HEADLESS) -append "$(ROOT_CMDLINE) console=ttyS0 init=/bin/cjsh"

# Graphical: opens a GTK window, kernel uses tty0/fbcon. Closer to what the
# Pi will look like over HDMI than the Ghostty-rendered serial view.
run-graphical: image
	$(QEMU_GRAPHICAL) -append "$(ROOT_CMDLINE) init=/init"

run-graphical-shell-only: image
	$(QEMU_GRAPHICAL) -append "$(ROOT_CMDLINE) init=/bin/cjsh"

# Short aliases.
run-g:  run-graphical
run-gs: run-graphical-shell-only

clean:
	$(MAKE) -C $(USERLAND) clean
	$(MAKE) -C $(DISK) clean
	rm -rf $(BUILD)
