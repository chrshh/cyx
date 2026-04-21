#ifndef ANSI_H
#define ANSI_H

#include <unistd.h>

/**
 * Check if terminal supports color
 * Prevents escape codes being dumped into log files or piped out
 */
#define TERM_IS_TTY() (isatty(STDOUT_FILENO))

// Reset
#define RESET "\033[0m"

// Regular Colors
#define BLACK "\033[30m"
#define RED "\033[31m"
#define GREEN "\033[32m"
#define YELLOW "\033[33m"
#define BLUE "\033[34m"
#define MAGENTA "\033[35m"
#define CYAN "\033[36m"
#define WHITE "\033[37m"

// Bright / bold colors
#define BLACK_BRIGHT "\033[90m"
#define RED_BRIGHT "\033[91m"
#define GREEN_BRIGHT "\033[92m"
#define YELLOW_BRIGHT "\033[93m"
#define BLUE_BRIGHT "\033[94m"
#define MAGENTA_BRIGHT "\033[95m"
#define CYAN_BRIGHT "\033[96m"
#define WHITE_BRIGHT "\033[97m"

// Background colors
#define BG_BLACK "\033[40m"
#define BG_RED "\033[41m"
#define BG_GREEN "\033[42m"
#define BG_YELLOW "\033[43m"
#define BG_BLUE "\033[44m"
#define BG_MAGENTA "\033[45m"
#define BG_CYAN "\033[46m"
#define BG_WHITE "\033[47m"

// Text Style
#define BOLD "\033[1m"
#define DIM "\033[2m"
#define ITALIC "\033[3m"
#define UNDERLINE "\033[4m"
#define BLINK "\033[5m"
#define INVERSE "\033[7m" // swap fg/bg colors
#define STRIKETHROUGH "\033[9m"

/**
 * Color + Reset + Style
 * Usage: printf(COLOR("failure", RED));
 * Usage: printf(COLOR("success", GREEN_BRIGHT));
 * */
#define COLOR(text, color) color text RESET
#define BOLD_COLOR(text, color) BOLD color text RESET

// Cursor Movement
#define CURSOR_UP(n) "\033[" #n "A"
#define CURSOR_DOWN(n) "\033[" #n "B"
#define CURSOR_RIGHT(n) "\033[" #n "C"
#define CURSOR_LEFT(n) "\033[" #n "D"
#define CURSOR_HIDE "\033[?25l"
#define CURSOR_SHOW "\033[?25h"
#define CURSOR_SAVE "\033[s"
#define CURSOR_RESTORE "\033[u"

// Semantic Alias
#define COLOR_ERROR RED_BRIGHT
#define COLOR_WARN YELLOW
#define COLOR_SUCCESS GREEN_BRIGHT
#define COLOR_INFO CYAN
#define COLOR_PROMPT BLUE_BRIGHT
#define COLOR_CMD WHITE_BRIGHT
#define COLOR_PATH CYAN_BRIGHT
#define COLOR_DEBUG DIM

// Terminal Control
#define CLEAR_SCREEN "\033[2J\033[H" // clear + move cursor to top
#define CLEAR_LINE "\033[2K\r"       // clear current line
#define CLEAR_LINE_RIGHT "\033[0K"   // clear from cursor to end of line

#endif
