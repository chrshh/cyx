FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    gcc \
    make \
    libreadline-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /cjyx

COPY cjhlib/ cjhlib/
COPY libcjyx/ libcjyx/
COPY cjsh/ cjsh/
COPY cmd/ cmd/
COPY Makefile .

RUN make clean && make all

# Put custom binaries on PATH
ENV PATH="/cjyx/cmd/bin:$PATH"

WORKDIR /cjyx/cjsh
CMD ["/cjyx/cjsh/cjsh"]
