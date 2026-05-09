.PHONY: all libs run static clean

CC = gcc

CJHLIB    = ../cjhlib
LIBCJYX   = ../libcjyx

CFLAGS  = -Wall -Wextra -Wpedantic -Wformat=2 -Wconversion -Wstrict-prototypes -Wmissing-prototypes -Wnull-dereference -O2 -I$(CJHLIB)/include -I$(LIBCJYX)/include $(EXTRA_FLAGS)
LDFLAGS = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib

SRCS = $(wildcard *.c)
BINS = $(patsubst %.c, bin/%, $(SRCS))

all: bin libs $(BINS)

# Static build for the disk image: the guest has no shared libs, so every
# binary that ships in rootfs/bin must be -static. Target-specific CFLAGS
# propagates down through `all` -> `$(BINS)` so each link adds -static.
static: CFLAGS += -static
static: all

bin:
	mkdir -p bin

libs:
	$(MAKE) -C $(CJHLIB) lib
	$(MAKE) -C $(LIBCJYX) lib

bin/%: %.c libs
	$(CC) $(CFLAGS) $< $(LDFLAGS) -o $@

# make run cmd=cat
run: bin/$(cmd)
	./bin/$(cmd)

clean:
	rm -rf bin
