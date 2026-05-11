# cjyx kernel

Custom-built Linux kernel for cjyx. The upstream source tree is gitignored
(2.6GB, fully reproducible from kernel.org). Only `.config` is tracked.

## Version

Linux **7.0.5** (mainline stable).

## Fetch + build

```sh
cd kernel
curl -O https://cdn.kernel.org/pub/linux/kernel/v7.x/linux-7.0.5.tar.xz
tar -xf linux-7.0.5.tar.xz
mv linux-7.0.5 src

# Drop our tracked .config in place (this file lives in the repo)
git checkout -- src/.config

# GCC 16 is stricter than the kernel's headers expect; this flag silences
# unterminated-string-initialization errors in ACPI code.
make -C src -j$(nproc) KCFLAGS="-Wno-error=unterminated-string-initialization" bzImage
```

The resulting bzImage lives at `src/arch/x86/boot/bzImage` and is consumed
by the root `Makefile` via the `KERNEL` variable.

## Critical config

These are the non-default knobs that cjyx depends on (all `=y`, built-in —
this kernel has no module loading):

- `DRM`, `DRM_VIRTIO_GPU`, `DRM_BOCHS` — DRM/KMS for virtio-gpu-pci
- `VIRTIO_PCI`, `VIRTIO_BLK` — virtio transport + block device
- `DEVTMPFS`, `DEVTMPFS_MOUNT` — auto-mount /dev
- `TMPFS` — /tmp, /run, /dev/shm
- `EXT4_FS` — rootfs + /home
- `INPUT_EVDEV` — keyboard/mouse for libinput

To regenerate from scratch: `make defconfig`, then `scripts/config --enable
<SYMBOL>` for each of the above, then `make olddefconfig`.
