.PHONY: all static clean

CC = gcc
CFLAGS = -Wall -Wextra -std=gnu11 -D_DEFALT_SOURCE -O2 $(EXTRA_FLAGS)

TARGET = asemics
SRC = main.c terminal.c input.c editor.c rows.c statbar.c commands.c syntax_hl.c motionfns.c search.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC)

static: CFLAGS += -static
static: $(TARGET)

clean:
	rm -f $(TARGET)
