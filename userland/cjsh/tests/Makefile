CC = gcc
CFLAGS = -Wall -Wextra -O0 -g -I../include -I../../cjhlib/include -I../../libcjyx/include

# Shell source objects (everything except cjsh.c which has main)
SHELL_SRC = ../src/lexer.c ../src/parser.c ../src/ast.c ../src/common.c ../src/builtins.c ../src/signals.c ../src/exec.c
SHELL_OBJ = $(SHELL_SRC:../src/%.c=%.o)

CJHLIB = ../../cjhlib
LIBCJYX = ../../libcjyx

LIBS = -L$(LIBCJYX) -L$(CJHLIB) -lcjyx -lmylib -lreadline

TESTS = test_lexer test_parser test_ast

all: $(TESTS)
	@echo ""
	@for t in $(TESTS); do echo "=== Running $$t ===" && ./$$t && echo "" || exit 1; done
	@echo "All unit tests passed."

test_lexer: test_lexer.c $(SHELL_OBJ) $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a
	$(CC) $(CFLAGS) -o $@ test_lexer.c $(SHELL_OBJ) $(LIBS)

test_parser: test_parser.c $(SHELL_OBJ) $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a
	$(CC) $(CFLAGS) -o $@ test_parser.c $(SHELL_OBJ) $(LIBS)

test_ast: test_ast.c $(SHELL_OBJ) $(CJHLIB)/libmylib.a $(LIBCJYX)/libcjyx.a
	$(CC) $(CFLAGS) -o $@ test_ast.c $(SHELL_OBJ) $(LIBS)

%.o: ../src/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(CJHLIB)/libmylib.a:
	$(MAKE) -C $(CJHLIB) lib

$(LIBCJYX)/libcjyx.a:
	$(MAKE) -C $(LIBCJYX) lib

integration: ../cjsh
	@echo "=== Running integration tests ==="
	@chmod +x test_exec.sh && ./test_exec.sh

../cjsh:
	$(MAKE) -C ..

clean:
	rm -f $(TESTS) *.o

.PHONY: all integration clean
