CC = gcc
CFLAGS = -Wall -Wextra -Wpedantic -Wformat=2 -Wconversion -Wstrict-prototypes -Wmissing-prototypes -Wnull-dereference -O2 -Iinclude -I../cjhlib/include -I../libcjyx/include $(EXTRA_FLAGS)

SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib
LIBCJYX = ../libcjyx

cjsh: $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline -o cjsh

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

$(LIBCJYX)/libcjyx.a:
	$(MAKE) -C $(LIBCJYX) lib

run: cjsh
	./cjsh

clean:
	rm -f src/*.o cjsh
