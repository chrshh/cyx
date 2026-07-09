#!/usr/bin/env python3
"""Build zonule (cjyx's Smithay Wayland compositor) inside a Debian-bookworm
Rust container and install the binary.

Same Rust-in-docker pattern as apps/squishy: the container's glibc (bookworm
2.36) matches the guest userland, and CARGO_TARGET_DIR points at a build/ dir so
host cargo runs (newer glibc) and the guest-targeted docker runs don't fight
over the same target/ directory.

zonule depends on smithay as a *git* dependency, so in addition to the crates.io
registry cache we mount a persistent cargo git cache — otherwise every build
re-clones smithay from scratch.
"""
import argparse
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/display/zonule
REPO_ROOT = SCRIPT_DIR.parents[2]                          # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "zonule-target"
# Shared, persistent cargo caches, mounted into the build container so deps are
# fetched once and only re-fetched when Cargo.lock changes.
REGISTRY_CACHE = BUILD_DIR / "cargo-registry"
GIT_CACHE = BUILD_DIR / "zonule-cargo-git"
IMAGE_TAG = "cjyx-zonule-builder"


def run(cmd):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("dest", type=Path, help="path to install the built zonule binary")
    args = ap.parse_args()

    TARGET_DIR.mkdir(parents=True, exist_ok=True)
    REGISTRY_CACHE.mkdir(parents=True, exist_ok=True)
    GIT_CACHE.mkdir(parents=True, exist_ok=True)

    run([
        "docker", "build",
        "--platform", "linux/arm64",
        "-t", IMAGE_TAG,
        "-f", str(SCRIPT_DIR / "Dockerfile.build"),
        str(SCRIPT_DIR),
    ])

    run([
        "docker", "run", "--rm",
        "--platform", "linux/arm64",
        "-v", f"{SCRIPT_DIR}:/work",
        "-v", f"{TARGET_DIR}:/target",
        "-v", f"{REGISTRY_CACHE}:/usr/local/cargo/registry",
        "-v", f"{GIT_CACHE}:/usr/local/cargo/git",
        "-e", "CARGO_TARGET_DIR=/target",
        "-w", "/work",
        IMAGE_TAG,
        "cargo", "build", "--release",
    ])

    binary = TARGET_DIR / "release" / "zonule"
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
