CC = gcc
CFLAGS = -Wall -Wextra -Iinclude -I../cjhlib/include

SRC = $(wildcard src/**/*.c src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib

lib: $(CJHLIB)/libmylib.a $(OBJ)
	ar rcs libcjyx.a $(OBJ)

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

tests: lib
	$(CC) $(CFLAGS) tests/test_fs.c -L. -lcjyx -L$(CJHLIB) -lmylib -o run_fs

run: lib tests
	./run_$(t)

all: lib tests
	./run_fs

clean:
	rm -f src/**/*.o src/*.o libcjyx.a run_*
