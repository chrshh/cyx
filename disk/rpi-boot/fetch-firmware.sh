#!/usr/bin/env bash
# Fetch the Raspberry Pi 4 VideoCore firmware blobs into this directory.
#
# The Pi 4 boots from SPI EEPROM (no bootcode.bin needed), then loads these two
# closed-source blobs off the FAT partition to bring up the SoC and hand off to
# our kernel:
#   start4.elf  — the GPU/second-stage firmware
#   fixup4.dat  — SDRAM partition fixups paired with start4.elf
#
# Run this once before `make sd-card`. Network access required.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE="https://github.com/raspberrypi/firmware/raw/master/boot"

for blob in start4.elf fixup4.dat; do
  echo "fetching $blob ..."
  curl -fsSL "$BASE/$blob" -o "$HERE/$blob"
done

echo "firmware ready in $HERE"
