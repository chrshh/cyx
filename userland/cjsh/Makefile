CC = gcc
CFLAGS = -Wall -Wextra -Wpedantic -Wformat=2 -Wconversion -Wstrict-prototypes -Wmissing-prototypes -Wnull-dereference -O2 -Iinclude -I../cjhlib/include -I../libcjyx/include $(EXTRA_FLAGS)

SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib
LIBCJYX = ../libcjyx

LIBS        = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline
STATIC_LIBS = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline -lncurses -ltinfo

.PHONY: static run sandbox clean

cjsh: $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) $(LIBS) -o cjsh

static: $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a $(OBJ)
	$(CC) $(CFLAGS) -static $(OBJ) $(STATIC_LIBS) -o cjsh

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

$(LIBCJYX)/libcjyx.a:
	$(MAKE) -C $(LIBCJYX) lib

run: cjsh
	./cjsh

# Fast dev loop: run the freshly-built ./cjsh under bubblewrap with a
# throwaway $HOME and /tmp. /usr and /etc are read-only-bound so external
# commands (ls, cat, …) still work and cjsh keeps its dynamic libreadline +
# libncursesw + libc deps. Your real home stays invisible to the shell, so
# misfires like `rm -rf ~` only blow up the temp dir, which evaporates on
# exit. Useful for iterating on builtins / parser / line-editing without
# rebuilding the full disk image or booting QEMU.
sandbox: cjsh
	@set -eu; \
	SANDBOX_HOME=$$(mktemp -d -t cjsh-sandbox.XXXXXX); \
	trap 'rm -rf "$$SANDBOX_HOME"' EXIT; \
	bwrap \
	  --unshare-all --share-net --die-with-parent \
	  --ro-bind /usr /usr \
	  --ro-bind /etc /etc \
	  --symlink usr/bin /bin \
	  --symlink usr/bin /sbin \
	  --symlink usr/lib /lib \
	  --symlink usr/lib /lib64 \
	  --dev /dev \
	  --proc /proc \
	  --tmpfs /tmp \
	  --bind "$$SANDBOX_HOME" /home/sandbox \
	  --ro-bind "$$(pwd)/cjsh" /cjsh \
	  --setenv HOME /home/sandbox \
	  --setenv USER sandbox \
	  --setenv PATH /usr/bin \
	  --setenv TERM "$$TERM" \
	  --chdir /home/sandbox \
	  /cjsh

clean:
	rm -f src/*.o cjsh
