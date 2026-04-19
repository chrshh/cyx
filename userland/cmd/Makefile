.PHONY: all libs run clean

CC = gcc

CJHLIB    = ../cjhlib
LIBCJYX   = ../libcjyx

CFLAGS  = -Wall -Wextra -I$(CJHLIB)/include -I$(LIBCJYX)/include
LDFLAGS = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib

SRCS = $(wildcard *.c)
BINS = $(patsubst %.c, bin/%, $(SRCS))

all: bin libs $(BINS)

bin:
	mkdir -p bin

libs:
	$(MAKE) -C $(CJHLIB) lib
	$(MAKE) -C $(LIBCJYX) lib

bin/%: %.c libs
	$(CC) $(CFLAGS) $< $(LDFLAGS) -o $@

# make run cmd=ccat
run: bin/$(cmd)
	./bin/$(cmd)

clean:
	rm -rf bin
