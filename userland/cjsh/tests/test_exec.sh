#!/bin/sh
# Integration tests for cjsh execution
# Runs the shell binary with piped input and checks stdout

CJSH=../cjsh
PASS=0
FAIL=0

assert_output() {
  desc="$1"
  input="$2"
  expected="$3"

  actual=$(echo "$input" | "$CJSH" 2>/dev/null | tail -1)

  if [ "$actual" = "$expected" ]; then
    PASS=$((PASS + 1))
  else
    FAIL=$((FAIL + 1))
    printf "  FAIL: %s\n    expected: \"%s\"\n    got:      \"%s\"\n" "$desc" "$expected" "$actual"
  fi
}

# Requires cjsh binary to exist
if [ ! -x "$CJSH" ]; then
  echo "ERROR: cjsh binary not found. Run 'make' first."
  exit 1
fi

echo "--- exec: integration tests ---"

# Simple command execution
assert_output "echo prints text" "echo hello" "hello"
assert_output "echo multiple words" "echo hello world" "hello world"

# Pipeline
assert_output "echo | cat passthrough" "echo hello | cat" "hello"
assert_output "echo | wc -w word count" "echo hello world | wc -w" "       2"

# Variable assignment and expansion
assert_output "set and echo var" 'FOO=testing
echo $FOO' "testing"

# Export and use
assert_output "export var" 'export BAR=exported
echo $BAR' "exported"

echo ""
TOTAL=$((PASS + FAIL))
echo "=== $TOTAL tests: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ] || exit 1
