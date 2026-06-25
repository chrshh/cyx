#!/usr/bin/env python3
"""Build palmer inside a Debian-bookworm Rust container and install the binary.

Unlike the standalone apps (squishy/dinky), palmer depends on the cfd/cg
command crates as libraries via path deps, so it must build from the Cargo
workspace root (userland/) rather than its own dir — the path deps point
outside apps/palmer/. We mount userland/ and build just `-p palmer`.

CARGO_TARGET_DIR points at build/palmer-target so host cargo runs (glibc-2.43
on Arch) and the guest-targeted docker runs (glibc 2.36) don't fight over the
same target/ directory.
"""
import argparse
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/apps/palmer
USERLAND = SCRIPT_DIR.parents[1]                          # userland (workspace root)
REPO_ROOT = SCRIPT_DIR.parents[2]                         # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "palmer-target"
IMAGE_TAG = "cjyx-palmer-builder"


def run(cmd):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("dest", type=Path, help="path to install the built palmer binary")
    args = ap.parse_args()

    TARGET_DIR.mkdir(parents=True, exist_ok=True)

    run([
        "docker", "build",
        "-t", IMAGE_TAG,
        "-f", str(SCRIPT_DIR / "Dockerfile.build"),
        str(SCRIPT_DIR),
    ])

    run([
        "docker", "run", "--rm",
        "-v", f"{USERLAND}:/work",
        "-v", f"{TARGET_DIR}:/target",
        "-e", "CARGO_TARGET_DIR=/target",
        "-w", "/work",
        IMAGE_TAG,
        "cargo", "build", "--release", "-p", "palmer",
    ])

    binary = TARGET_DIR / "release" / "palmer"
    if not binary.exists():
        print(f"build did not produce {binary}", file=sys.stderr)
        return 1

    args.dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(binary, args.dest)
    args.dest.chmod(0o755)
    print(f"installed {binary} -> {args.dest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
