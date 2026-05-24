FROM debian:bookworm-slim

# Build-time toolchain + cjyx build deps. Headers needed to compile cjsh,
# tinywl, etc. inside this image.
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

# Runtime userland for the guest OS. apt-get pulls hundreds of transitive
# deps; the ones listed are the "intentional" ones for each ecosystem we
# want to support inside cjyx. Grouped by purpose for legibility.
RUN apt-get update && apt-get install -y --no-install-recommends \
    # OpenGL / Mesa (llvmpipe software rasterizer + GBM)
    libgl1-mesa-dri \
    libglx-mesa0 \
    libegl1 \
    libgles2 \
    libgbm1 \
    mesa-utils \
    # Wayland clients (server-side libs already covered by libwlroots-dev)
    libwayland-cursor0 \
    libwayland-egl1 \
    libdecor-0-0 \
    # GTK 4 + assets
    libgtk-4-1 \
    libgtk-3-0 \
    gsettings-desktop-schemas \
    adwaita-icon-theme \
    hicolor-icon-theme \
    shared-mime-info \
    # Qt 6 (GUI + Wayland platform plugin)
    libqt6gui6 \
    libqt6widgets6 \
    qt6-wayland \
    qt6-qpa-plugins \
    # GLFW (raylib uses its own Wayland code on PLATFORM_DESKTOP, but other
    # apps want GLFW; the plain package has both Wayland + X11 support)
    libglfw3 \
    # Audio (raylib uses miniaudio which can speak ALSA directly)
    libasound2 \
    # D-Bus (GTK and Qt both want a session bus)
    dbus \
    # Fonts
    fonts-dejavu-core \
    fontconfig \
    # Wayland-native terminal emulator (foot)
    foot \
    # X11 fallback for legacy apps via Xwayland
    xwayland \
    # Locale support (apps misbehave without an installed locale)
    locales \
    # TUI substrate: terminfo database + ncurses runtime
    ncurses-base \
    ncurses-bin \
    libncursesw6 \
    # Generic runtime deps GTK/Qt apps tend to dlopen
    libudev1 \
    && rm -rf /var/lib/apt/lists/*

# Generate en_US.UTF-8 locale so glibc has something to use beyond C.
RUN sed -i 's/# en_US.UTF-8/en_US.UTF-8/' /etc/locale.gen && locale-gen

# raylib isn't packaged for Debian, so build the Wayland-flavored shared
# library from source. Installs to /usr/local/lib/libraylib.so and
# /usr/local/include/raylib.h. Build-time deps: git, GL/EGL dev headers,
# libdecor-dev (Wayland window decorations). Wayland/xkbcommon dev
# headers are already present from the cjyx build deps above.
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
# Only textr ships through the C image — squishy has its own Rust toolchain
# image (see apps/squishy/Dockerfile.build) and copying apps/squishy here
# would drag in its multi-hundred-MB target/ dir.
COPY apps/textr/ apps/textr/
COPY Makefile .

RUN make clean && make $BUILD_TARGET EXTRA_FLAGS="$EXTRA_FLAGS"

# Put custom binaries on PATH
ENV PATH="/cjyx/cmd/bin:$PATH"

WORKDIR /cjyx/cjsh
CMD ["/cjyx/cjsh/cjsh"]
