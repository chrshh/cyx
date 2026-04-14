CC = gcc
CFLAGS = -Wall -Wextra -Iinclude

SRC = $(wildcard src/ds/*.c src/core/*.c)
OBJ = $(SRC:.c=.o)

lib: $(OBJ)
	ar rcs libmylib.a $(OBJ)

tests: lib
	$(CC) $(CFLAGS) tests/test_hashmap.c -L. -lmylib -o run_hashmap
	$(CC) $(CFLAGS) tests/test_array_int.c -L. -lmylib -o run_array_int
	$(CC) $(CFLAGS) tests/test_linkedlist.c -L. -lmylib -o run_linkedlist

# make leet file=leetcode/arrays/two_sum.c
leet:
	$(CC) $(CFLAGS) $(file) -L. -lmylib -o run_leet

# make run t=linkedlist    -> builds lib + tests, runs run_linkedlist
# make run t=array_int     -> builds lib + tests, runs run_array_int
# make run t=leet file=leetcode/arrays/two_sum.c
run: lib tests
	./run_$(t)

# make all -> builds lib + tests, runs all test binaries
all: lib tests
	./run_hashmap
	./run_array_int
	./run_linkedlist

clean:
	rm -f src/ds/*.o src/core/*.o libmylib.a run_*
