CC = gcc
CFLAGS = -Wall -Wextra -I../cjhlib/include -I../libcjyx/include

SRC = $(wildcard src/*.c)
OBJ = $(SRC:.c=.o)

CJHLIB = ../cjhlib
LIBCJYX = ../libcjyx

cjsh: $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -L$(LIBCJYX) -lcjyx -L$(CJHLIB) -lmylib -lreadline -o cjsh

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

$(LIBCJYX)/libcjyx.a:
	$(MAKE) -C $(LIBCJYX) lib

run: cjsh
	./cjsh

clean:
	rm -f src/*.o cjsh
