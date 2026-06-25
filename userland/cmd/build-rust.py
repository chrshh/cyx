#!/usr/bin/env python3
"""Build every Rust command crate (the workspace members under cmd/) inside a
Debian-bookworm Rust container and install each release binary into a
destination directory.

The disk Makefile's stage-cmd-rs target then iterates that directory and
copies each executable into rootfs/bin, mirroring how stage-cmd handles the
C commands in build/cmd/.

The Cargo workspace root is userland/Cargo.toml (one level up from this
script), so we mount the whole userland/ tree as the build context and let
`cargo build --workspace` build exactly the members listed there — i.e. the
Rust command crates, not the apps/ (which are excluded from the workspace).

CARGO_TARGET_DIR points at build/cmd-rs-target so that host cargo runs
(glibc-2.43 on Arch) and the guest-targeted docker runs (glibc 2.36) do not
fight over the same target/ directory.
"""
import argparse
import shutil
import stat
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/cmd
USERLAND = SCRIPT_DIR.parent                              # userland (workspace root)
REPO_ROOT = USERLAND.parent                              # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "cmd-rs-target"
IMAGE_TAG = "cjyx-cmd-rs-builder"


def run(cmd):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True)


def collect_executables(release_dir: Path):
    """Return executable files at the top level of target/release/.

    cargo writes one binary per bin crate directly into target/release/.
    Subdirectories (deps/, build/, .fingerprint/, examples/, incremental/)
    contain intermediate artifacts we don't want. We also skip *.d depfiles
    and dotfiles.
    """
    bins = []
    for entry in sorted(release_dir.iterdir()):
        if not entry.is_file():
            continue
        if entry.name.startswith(".") or entry.suffix == ".d":
            continue
        mode = entry.stat().st_mode
        if not (mode & stat.S_IXUSR):
            continue
        bins.append(entry)
    return bins


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("dest", type=Path,
                    help="directory to install the built command binaries into")
    args = ap.parse_args()

    dest_dir: Path = args.dest
    dest_dir.mkdir(parents=True, exist_ok=True)

    TARGET_DIR.mkdir(parents=True, exist_ok=True)

    run([
        "docker", "build",
        "-t", IMAGE_TAG,
        "-f", str(SCRIPT_DIR / "Dockerfile.rust"),
        str(SCRIPT_DIR),
    ])

    run([
        "docker", "run", "--rm",
        "-v", f"{USERLAND}:/work",
        "-v", f"{TARGET_DIR}:/target",
        "-e", "CARGO_TARGET_DIR=/target",
        "-w", "/work",
        IMAGE_TAG,
        "cargo", "build", "--release", "--workspace",
    ])

    release_dir = TARGET_DIR / "release"
    if not release_dir.is_dir():
        print(f"build did not produce {release_dir}", file=sys.stderr)
        return 1

    bins = collect_executables(release_dir)
    if not bins:
        print(f"no executables found in {release_dir}", file=sys.stderr)
        return 1

    installed = []
    for binary in bins:
        target = dest_dir / binary.name
        shutil.copy2(binary, target)
        target.chmod(0o755)
        installed.append(target)

    print(f"cmd-rs: installed {len(installed)} binary(ies) into {dest_dir}:")
    for path in installed:
        print(f"  {path.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
