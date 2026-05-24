#!/usr/bin/env python3
"""Build squishy inside a Debian-bookworm Rust container and install the binary.

Builds with CARGO_TARGET_DIR pointing at the top-level build/squishy-target
directory so that host cargo runs (which produce glibc-2.43 artifacts) and
the guest-targeted docker runs (glibc 2.36) don't fight over the same
target/ directory.
"""
import argparse
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/apps/squishy
REPO_ROOT = SCRIPT_DIR.parents[2]                          # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "squishy-target"
IMAGE_TAG = "cjyx-squishy-builder"


def run(cmd):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("dest", type=Path, help="path to install the built squishy binary")
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
        "-v", f"{SCRIPT_DIR}:/work",
        "-v", f"{TARGET_DIR}:/target",
        "-e", "CARGO_TARGET_DIR=/target",
        "-w", "/work",
        IMAGE_TAG,
        "cargo", "build", "--release",
    ])

    binary = TARGET_DIR / "release" / "squishy"
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
