#!/usr/bin/env bash
# Build a flashable Raspberry Pi 4 SD-card image WITHOUT root or loop devices.
#
# Layout (MBR):
#   p1  FAT32  boot   — firmware (start4.elf/fixup4.dat), kernel Image, dtb,
#                       config.txt, cmdline.txt   (what the VideoCore reads)
#   p2  ext4   root   — the staged rootfs           (root=/dev/mmcblk0p2)
#
# We assemble it by building each filesystem into its own regular file (mtools
# for FAT, `mke2fs -d` for ext4), writing an MBR partition table with sfdisk,
# then dd'ing the two filesystem blobs to their partition offsets. None of this
# needs privileges.
#
# Inputs (env vars, set by the top-level Makefile's sd-card target):
#   KERNEL_IMAGE  path to arch/arm64/boot/Image
#   DTB           path to bcm2711-rpi-4-b.dtb
#   RPIBOOT       dir holding config.txt, cmdline.txt, start4.elf, fixup4.dat
#   ROOTFS        staged root filesystem tree (disk/rootfs)
#   OUT           output image path (build/sdcard.img)
set -euo pipefail

: "${KERNEL_IMAGE:?}" "${DTB:?}" "${RPIBOOT:?}" "${ROOTFS:?}" "${OUT:?}"

SECTOR=512
ALIGN_SECTORS=2048                       # 1 MiB alignment
BOOT_MB=256
BOOT_SECTORS=$(( BOOT_MB * 1024 * 1024 / SECTOR ))
BOOT_START=$ALIGN_SECTORS
ROOT_START=$(( BOOT_START + BOOT_SECTORS ))

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# --- p1: FAT32 boot filesystem ---
BOOT_IMG="$WORK/boot.img"
truncate -s "${BOOT_MB}M" "$BOOT_IMG"
mkfs.vfat -F 32 -n CJYXBOOT "$BOOT_IMG" >/dev/null

for blob in start4.elf fixup4.dat; do
  if [ ! -f "$RPIBOOT/$blob" ]; then
    echo "error: missing firmware '$RPIBOOT/$blob'." >&2
    echo "       run $RPIBOOT/fetch-firmware.sh first." >&2
    exit 1
  fi
done

mcopy -i "$BOOT_IMG" "$KERNEL_IMAGE"        ::Image
mcopy -i "$BOOT_IMG" "$DTB"                 ::bcm2711-rpi-4-b.dtb
mcopy -i "$BOOT_IMG" "$RPIBOOT/config.txt"  ::config.txt
mcopy -i "$BOOT_IMG" "$RPIBOOT/cmdline.txt" ::cmdline.txt
mcopy -i "$BOOT_IMG" "$RPIBOOT/start4.elf"  ::start4.elf
mcopy -i "$BOOT_IMG" "$RPIBOOT/fixup4.dat"  ::fixup4.dat

# --- p2: ext4 root filesystem, sized to rootfs usage + 512 MiB slack ---
ROOT_IMG="$WORK/root.img"
ROOT_KB="$(du -s -k "$ROOTFS" | cut -f1)"
ROOT_MB=$(( ROOT_KB / 1024 + 512 ))
truncate -s "${ROOT_MB}M" "$ROOT_IMG"
# -O ^metadata_csum: mke2fs -d directory pre-population doesn't reserve room for
# directory-leaf checksums, which makes the guest kernel error the fs on first
# read ("No space for directory leaf checksum"). Disable it (see disk/Makefile).
mkfs.ext4 -F -q -O ^metadata_csum -L cjyxroot -d "$ROOTFS" "$ROOT_IMG"
ROOT_SECTORS=$(( ROOT_MB * 1024 * 1024 / SECTOR ))

# --- assemble the partitioned image ---
TOTAL_SECTORS=$(( ROOT_START + ROOT_SECTORS ))
truncate -s $(( TOTAL_SECTORS * SECTOR )) "$OUT"

# MBR table: p1 FAT32-LBA (0c) bootable, p2 Linux (83).
sfdisk "$OUT" >/dev/null <<EOF
label: dos
unit: sectors
${BOOT_START},${BOOT_SECTORS},0c,*
${ROOT_START},${ROOT_SECTORS},83
EOF

dd if="$BOOT_IMG" of="$OUT" bs=$SECTOR seek=$BOOT_START conv=notrunc status=none
dd if="$ROOT_IMG" of="$OUT" bs=$SECTOR seek=$ROOT_START conv=notrunc status=none

echo "wrote $OUT"
echo "  p1 FAT32 boot: ${BOOT_MB} MiB   p2 ext4 root: ${ROOT_MB} MiB   total: $(( TOTAL_SECTORS * SECTOR / 1024 / 1024 )) MiB"
echo "flash with: dd if=$OUT of=/dev/sdX bs=4M conv=fsync status=progress"
