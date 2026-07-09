# Two-stage build for the cjyx aarch64 (Raspberry Pi 4 / BCM2711) userland.
#
#   builder  — full toolchain + -dev headers; compiles cjsh/display/cmd and
#              builds raylib from source. Heavy, never shipped.
#   runtime  — ONLY runtime packages. This is what `docker export` turns into
#              the guest rootfs, so it must carry no compiler, no -dev, no
#              gdb/git/perl/python cruft. The built binaries are copied in from
#              the builder stage.
#
# The target platform (linux/arm64) is set by `docker build --platform` in the
# top-level Makefile, so every produced binary and every Debian lib is aarch64
# (built via qemu-user/binfmt on an x86_64 host).

# =============================================================================
# Stage 1: builder
# =============================================================================
FROM debian:bookworm-slim AS builder

# Build-time toolchain + cjyx build deps (headers to compile cjsh, display/
# tinywl, cmd). These stay in the builder and never reach the guest.
RUN apt-get update && apt-get install -y \
    gcc \
    make \
    pkg-config \
    libreadline-dev \
    libncurses-dev \
    cpio \
    gdb \
    libwlroots-dev \
    wayland-protocols \
    libwayland-dev \
    libxkbcommon-dev \
    libdrm-dev \
    libgbm-dev \
    libinput-dev \
    libudev-dev \
    udev \
    libpixman-1-dev \
    libseat-dev \
    seatd \
    && rm -rf /var/lib/apt/lists/*

# raylib isn't packaged for Debian, so build the Wayland-flavored shared lib
# from source → /usr/local/lib/libraylib.so (+ header). Build-time deps: git,
# GL/EGL dev headers, libdecor-dev, and the X dev headers GLFW's X11 backend
# wants. Wayland/xkbcommon dev headers are already present above.
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    libgles-dev \
    libegl-dev \
    libdecor-0-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxi-dev \
    libx11-dev \
    xkb-data \
    && git clone --depth=1 --branch=5.5 https://github.com/raysan5/raylib.git /tmp/raylib \
    && cd /tmp/raylib/src \
    && make PLATFORM=PLATFORM_DESKTOP RAYLIB_LIBTYPE=SHARED USE_WAYLAND_DISPLAY=TRUE \
    && make install RAYLIB_LIBTYPE=SHARED \
    && cd / && rm -rf /tmp/raylib \
    && rm -rf /var/lib/apt/lists/*

ARG EXTRA_FLAGS=""
ENV EXTRA_FLAGS=${EXTRA_FLAGS}

# Pick which top-level userland target to build (e.g. all, static).
ARG BUILD_TARGET=all

WORKDIR /cjyx

COPY cjhlib/ cjhlib/
COPY libcjyx/ libcjyx/
COPY cjsh/ cjsh/
COPY cmd/ cmd/
COPY display/ display/
COPY Makefile .

RUN make clean && make $BUILD_TARGET EXTRA_FLAGS="$EXTRA_FLAGS"

# =============================================================================
# Stage 2: runtime (this is what becomes the guest rootfs)
# =============================================================================
FROM debian:bookworm-slim AS runtime

# Runtime userland only — no toolchain, no -dev, and (per the bloat pass) no
# GTK/Qt/foot. On arm64, libgl1-mesa-dri ships the v3d/vc4 Gallium drivers for
# the real Pi GPU plus llvmpipe for software rendering. --no-install-recommends
# keeps the transitive set tight; Depends: are still pulled automatically.
RUN apt-get update && apt-get install -y --no-install-recommends \
    # OpenGL / Mesa (llvmpipe software + v3d/vc4 hardware + GBM)
    libgl1-mesa-dri \
    libglx-mesa0 \
    libegl1 \
    libgles2 \
    libgbm1 \
    # Wayland client libs (server-side comes with libwlroots10)
    libwayland-client0 \
    libwayland-cursor0 \
    libwayland-egl1 \
    libdecor-0-0 \
    # Compositor runtime: wlroots + its seat/input/render deps
    libwlroots10 \
    seatd \
    libinput10 \
    # cjsh links readline (pulls libtinfo/ncurses runtime)
    libreadline8 \
    # GLFW + X client libs raylib's desktop backend may dlopen
    libglfw3 \
    libx11-6 \
    libxcursor1 \
    libxrandr2 \
    libxinerama1 \
    libxi6 \
    # Audio (raylib/miniaudio speaks ALSA directly)
    libasound2 \
    # D-Bus session bus
    dbus \
    # Fonts
    fonts-dejavu-core \
    fontconfig \
    # X11 fallback for legacy apps via Xwayland
    xwayland \
    # Locale support (apps misbehave without an installed locale)
    locales \
    # TUI substrate: terminfo database + ncurses runtime
    ncurses-base \
    ncurses-bin \
    libncursesw6 \
    # Device manager (systemd-udevd + udevadm; cinit coldplugs via these)
    udev \
    libudev1 \
    && rm -rf /var/lib/apt/lists/*

# Generate en_US.UTF-8 so glibc has more than the C locale.
RUN sed -i 's/# en_US.UTF-8/en_US.UTF-8/' /etc/locale.gen && locale-gen

# Pull the built artifacts over from the builder. The /cjyx tree is what the
# top-level Makefile's `docker cp` targets extract (it's excluded from the
# rootfs export); libraylib.so is a genuine runtime graphics lib and DOES ship.
COPY --from=builder /cjyx /cjyx
COPY --from=builder /usr/local/lib/libraylib.so* /usr/local/lib/

# Put custom binaries on PATH.
ENV PATH="/cjyx/cmd/bin:$PATH"

WORKDIR /cjyx/cjsh
CMD ["/cjyx/cjsh/cjsh"]
