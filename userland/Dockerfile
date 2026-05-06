FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    gcc \
    make \
    libreadline-dev \
    libncurses-dev \
    cpio \
    gdb \
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
COPY Makefile .

RUN make clean && make $BUILD_TARGET EXTRA_FLAGS="$EXTRA_FLAGS"

# Put custom binaries on PATH
ENV PATH="/cjyx/cmd/bin:$PATH"

WORKDIR /cjyx/cjsh
CMD ["/cjyx/cjsh/cjsh"]
