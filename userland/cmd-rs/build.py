#!/usr/bin/env python3
"""Build every crate in the cmd-rs workspace inside a Debian-bookworm Rust
container and install each release binary into a destination directory.

The disk Makefile's stage-cmd-rs target then iterates that directory and
copies each executable into rootfs/bin, mirroring how stage-cmd handles
the C commands in build/cmd/.

CARGO_TARGET_DIR points at build/cmd-rs-target so that host cargo runs
(glibc-2.43 on Arch) and the guest-targeted docker runs (glibc 2.36) do
not fight over the same target/ directory.
"""
import argparse
import shutil
import stat
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/cmd-rs
REPO_ROOT = SCRIPT_DIR.parents[2]                          # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "cmd-rs-target"
IMAGE_TAG = "cjyx-cmd-rs-builder"


def run(cmd):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True)


def discover_member_crates():
    """Return a list of (crate_dir, crate_name) for every member crate."""
    crates = []
    for manifest in sorted(SCRIPT_DIR.glob("*/Cargo.toml")):
        crate_dir = manifest.parent
        if crate_dir.name == "target":
            continue
        crates.append((crate_dir, crate_dir.name))
    return crates


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

    crates = discover_member_crates()
    if not crates:
        print("cmd-rs: no member crates yet — nothing to build.")
        print("       create one with: cd userland/cmd-rs && "
              "cargo new --vcs none --bin <name>")
        return 0

    print(f"cmd-rs: found {len(crates)} crate(s): "
          f"{', '.join(name for _, name in crates)}")

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
