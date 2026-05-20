CC = gcc
CFLAGS = -Wall -Wextra -std=gnu11 -D_DEFALT_SOURCE -O2

TARGET = textr
SRC = main.c terminal.c input.c editor.c rows.c statbar.c commands.c syntax_hl.c motionfns.c search.c

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC)

clean:
	rm -f $(TARGET)
