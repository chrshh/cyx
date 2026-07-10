#!/usr/bin/env python3
"""Build zonule (cjyx's Smithay Wayland compositor) for aarch64 by NATIVE
cross-compilation, and install the binary.

This used to run `cargo build` inside an arm64 Docker container, which meant
rustc itself executed under qemu-user emulation — correct, but slow (the whole
Smithay dependency graph gets compiled by an emulated compiler). Now rustc runs
NATIVELY on the x86_64 host and merely *targets* aarch64
(`aarch64-unknown-linux-gnu`), linking with the host's `aarch64-linux-gnu-gcc`.
That is the compile-time win: the compiler is native, build scripts and
proc-macros compile and run as native x86 code, and only the final object code
is aarch64.

The catch is glibc. The host's cross-gcc ships a bleeding-edge glibc (2.43), but
the guest userland is Debian bookworm (glibc 2.36); a binary linked against 2.43
would fail to start on the guest with "version `GLIBC_2.4x' not found". So we do
NOT link against the host cross toolchain's own libraries. Instead we extract a
bookworm aarch64 *sysroot* from the very same builder image we used to compile
inside before (rust:1-bookworm + Smithay's native deps) and aim the linker and
pkg-config at it. The result links against glibc 2.36 and the exact bookworm .so
versions the guest actually has.

Requirements on the host: `rustup` (the aarch64 std target is added automatically
if missing), `aarch64-linux-gnu-gcc`, `pkg-config`, and `docker` (only to source
the sysroot — the compile no longer runs in it).
"""
import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent              # userland/display/zonule
REPO_ROOT = SCRIPT_DIR.parents[2]                          # repo root
BUILD_DIR = REPO_ROOT / "build"
TARGET_DIR = BUILD_DIR / "zonule-target"
# Bookworm aarch64 sysroot extracted from the builder image; the linker and
# pkg-config resolve all system libs, headers, crt objects and .pc files here.
SYSROOT_DIR = BUILD_DIR / "zonule-sysroot"
IMAGE_TAG = "cjyx-zonule-builder"

TARGET_TRIPLE = "aarch64-unknown-linux-gnu"
MULTIARCH = "aarch64-linux-gnu"                            # Debian multiarch dir name
CROSS_GCC = "aarch64-linux-gnu-gcc"


def run(cmd, **kw):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True, **kw)


def capture(cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout.strip()


def ensure_rust_target():
    """Add the aarch64 std target to the default rustup toolchain if it isn't
    already installed. Idempotent and fast when present."""
    installed = capture(["rustup", "target", "list", "--installed"]).split()
    if TARGET_TRIPLE not in installed:
        run(["rustup", "target", "add", TARGET_TRIPLE])


def build_builder_image():
    """Build the bookworm image that carries Smithay's native deps. We no longer
    compile in it — it's just the source of the sysroot below — but building it
    is cheap when the Docker layer cache is warm."""
    run([
        "docker", "build",
        "--platform", "linux/arm64",
        "-t", IMAGE_TAG,
        "-f", str(SCRIPT_DIR / "Dockerfile.build"),
        str(SCRIPT_DIR),
    ])


def extract_sysroot():
    """Extract /usr and /lib (arm64, bookworm) from the builder image into
    SYSROOT_DIR. Cached: only re-extracted when the image ID changes, so day to
    day this is a no-op."""
    image_id = capture(["docker", "image", "inspect", "--format", "{{.Id}}", IMAGE_TAG])
    stamp = SYSROOT_DIR / ".image-id"
    if stamp.exists() and stamp.read_text().strip() == image_id and (SYSROOT_DIR / "usr").exists():
        print(f"sysroot up to date ({image_id[:19]}...)", flush=True)
        return

    print("extracting bookworm aarch64 sysroot from builder image ...", flush=True)
    if SYSROOT_DIR.exists():
        shutil.rmtree(SYSROOT_DIR)
    SYSROOT_DIR.mkdir(parents=True)

    cid = capture(["docker", "create", "--platform", "linux/arm64", IMAGE_TAG])
    try:
        # Stream the container filesystem and unpack only /usr and /lib — the
        # arm64 libraries, headers, crt objects and .pc files we link against.
        # (On usrmerge bookworm every real file lives under /usr, and /lib is a
        # symlink into it, so selecting these two top-level dirs captures every
        # hardlink target — no "Cannot stat" aborts like the --exclude approach.)
        export = subprocess.Popen(["docker", "export", cid], stdout=subprocess.PIPE)
        tar = subprocess.Popen(
            ["tar", "-x", "-C", str(SYSROOT_DIR), "usr", "lib"],
            stdin=export.stdout,
        )
        export.stdout.close()
        tar.communicate()
        export.wait()
        if export.returncode != 0 or tar.returncode != 0:
            print("sysroot extraction failed", file=sys.stderr)
            sys.exit(1)
    finally:
        subprocess.run(["docker", "rm", cid], stdout=subprocess.DEVNULL)

    # The Rust toolchain baked into the builder image is dead weight in a
    # sysroot (we link with the host's rustc/gcc, not these) — drop the biggest
    # offenders to keep the extracted tree lean.
    for junk in ("usr/local/cargo", "usr/local/rustup"):
        shutil.rmtree(SYSROOT_DIR / junk, ignore_errors=True)

    stamp.write_text(image_id)


def ensure_writable_target():
    """The old emulated builds ran cargo as root inside the container, leaving
    root-owned files in TARGET_DIR that the now-native (unprivileged) build
    can't overwrite. If the dir isn't writable, wipe it via a throwaway root
    container — those artifacts (target/{debug,release}, no triple subdir) are
    useless to the native build anyway. Only fires on the one-time migration;
    once the native build owns the dir this is a no-op and the incremental cache
    is preserved."""
    # The stale emulated artifacts sit in root-owned top-level debug/ and
    # release/ subdirs (the emulated build had no --target, so it wrote there
    # rather than under a triple subdir). Detect the dir itself or any child not
    # owned by us.
    uid = os.getuid()
    if not TARGET_DIR.exists():
        TARGET_DIR.mkdir(parents=True)
        return
    foreign = TARGET_DIR.stat().st_uid != uid or any(
        child.stat().st_uid != uid for child in TARGET_DIR.iterdir()
    )
    if not foreign:
        return
    print("resetting root-owned target dir left by old emulated builds ...", flush=True)
    # The container only removes (it runs as root, so it can); the host then
    # recreates the dir so it's owned by us, not by container-root.
    run([
        "docker", "run", "--rm",
        "-v", f"{BUILD_DIR}:/b",
        IMAGE_TAG,
        "rm", "-rf", "/b/zonule-target",
    ])
    TARGET_DIR.mkdir(parents=True, exist_ok=True)


def cargo_env():
    """Environment that makes host cargo cross-compile against the sysroot."""
    s = str(SYSROOT_DIR)
    ma = f"{s}/usr/lib/{MULTIARCH}"          # crt objects + arm64 .so live here
    lib_ma = f"{s}/lib/{MULTIARCH}"
    # The Arch cross-gcc doesn't know Debian's multiarch layout, so we spell out
    # the sysroot, the startfile dir (-B, for crt1/crti/crtn), and the library
    # search + rpath-link paths explicitly. -static-libgcc avoids a runtime
    # dependency on the host gcc-15 libgcc_s (guest only has bookworm's gcc-12).
    link_args = [
        f"--sysroot={s}",
        f"-B{ma}",
        f"-L{ma}",
        f"-L{lib_ma}",
        f"-Wl,-rpath-link,{ma}",
        f"-Wl,-rpath-link,{lib_ma}",
        "-static-libgcc",
    ]
    rustflags = " ".join(f"-Clink-arg={a}" for a in link_args)

    env = os.environ.copy()
    triple_env = TARGET_TRIPLE.upper().replace("-", "_")    # AARCH64_UNKNOWN_LINUX_GNU
    triple_cc = TARGET_TRIPLE.replace("-", "_")             # aarch64_unknown_linux_gnu
    env[f"CARGO_TARGET_{triple_env}_LINKER"] = CROSS_GCC
    env[f"CARGO_TARGET_{triple_env}_RUSTFLAGS"] = rustflags
    env["CARGO_TARGET_DIR"] = str(TARGET_DIR)

    # pkg-config must resolve the arm64 .pc files from the sysroot (and prefix
    # their -I/-L with it), not read the host's x86 ones. PKG_CONFIG_LIBDIR
    # replaces the search path outright so host .pc files can't leak in.
    env["PKG_CONFIG_ALLOW_CROSS"] = "1"
    env["PKG_CONFIG_SYSROOT_DIR"] = s
    env["PKG_CONFIG_LIBDIR"] = ":".join([
        f"{ma}/pkgconfig",
        f"{s}/usr/lib/pkgconfig",
        f"{s}/usr/share/pkgconfig",
    ])

    # If any build script compiles C via the `cc` crate, target aarch64 against
    # the sysroot. Defensive BINDGEN args in case a crate regenerates bindings
    # (harmless when bindgen isn't invoked — the FFI crates ship pre-generated).
    env[f"CC_{triple_cc}"] = CROSS_GCC
    env[f"CFLAGS_{triple_cc}"] = f"--sysroot={s}"
    env["BINDGEN_EXTRA_CLANG_ARGS"] = f"--sysroot={s} -I{s}/usr/include -I{ma}"
    return env


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("dest", type=Path, help="path to install the built zonule binary")
    ap.add_argument(
        "--dev",
        action="store_true",
        help="build the fast iteration profile (no LTO) instead of --release. "
        "Much quicker recompiles; the binary is unoptimized + larger.",
    )
    args = ap.parse_args()

    BUILD_DIR.mkdir(parents=True, exist_ok=True)
    TARGET_DIR.mkdir(parents=True, exist_ok=True)

    ensure_rust_target()
    build_builder_image()
    extract_sysroot()
    ensure_writable_target()

    cargo_build = ["cargo", "build", "--target", TARGET_TRIPLE]
    if not args.dev:
        cargo_build.append("--release")
    profile_dir = "debug" if args.dev else "release"

    run(cargo_build, cwd=str(SCRIPT_DIR), env=cargo_env())

    binary = TARGET_DIR / TARGET_TRIPLE / profile_dir / "zonule"
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
