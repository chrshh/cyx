CC = gcc
CFLAGS = -Wall -Wextra -Iinclude -I../cjhlib/include

SRC = $(wildcard src/**/*.c src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib

lib: $(CJHLIB)/libmylib.a $(OBJ)
	ar rcs libcjyx.a $(OBJ)

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

clean:
	rm -f src/**/*.o src/*.o libcjyx.a
