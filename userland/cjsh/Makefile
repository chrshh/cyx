CC = gcc
CFLAGS = -Wall -Wextra -Wpedantic -Wformat=2 -Wconversion -Wstrict-prototypes -Wmissing-prototypes -Wnull-dereference -O2 -Iinclude -I../cjhlib/include -I../libcjyx/include $(EXTRA_FLAGS)

SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib
LIBCJYX = ../libcjyx

LIBS        = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline
STATIC_LIBS = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline -lncurses -ltinfo

.PHONY: static run clean

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

clean:
	rm -f src/*.o cjsh
