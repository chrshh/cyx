#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
IntArr RemoveDuplicates(IntArr input) {
  IntArr arr = CreateIntArr();
  if (input.length == 0) {
    return input;
  }

  for (size_t i = 0; i < input.length; i++) {
    int curVal = input.data[i];
    int present = IASearch(arr, curVal);
    if (present == 0) {
      arr = IAAppend(arr, curVal);
    }
  }

  return arr;
}

// ---------- Utilities ----------

// print array
void PrintArr(IntArr arr) {
  printf("[");
  for (size_t i = 0; i < arr.length; i++) {
    printf("%d", arr.data[i]);
    if (i < arr.length - 1)
      printf(", ");
  }
  printf("]");
}

// compare arrays
int ArraysEqual(IntArr a, IntArr b) {
  if (a.length != b.length)
    return 0;

  for (size_t i = 0; i < a.length; i++) {
    if (a.data[i] != b.data[i])
      return 0;
  }
  return 1;
}

// free array
void FreeArr(IntArr arr) { free(arr.data); }

// ---------- Tests ----------

void RunTest(int *raw, size_t rawLen, int *expectedRaw, size_t expectedLen,
             int testNum) {
  IntArr input = CreateIntArrFromData(raw, rawLen);
  IntArr expected = CreateIntArrFromData(expectedRaw, expectedLen);

  IntArr result = RemoveDuplicates(input);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  PrintArr(input);
  printf("\n");
  printf("Expected: ");
  PrintArr(expected);
  printf("\n");
  printf("Result:   ");
  PrintArr(result);
  printf("\n");
  printf("PASS: %s\n\n", ArraysEqual(result, expected) ? "YES" : "NO");

  FreeArr(input);
  FreeArr(expected);
  FreeArr(result);
}

int main() {
  printf("=== Remove Duplicates Tests ===\n\n");

  // Test 1
  int raw1[] = {3, 1, 3, 2, 1, 4};
  int expected1[] = {3, 1, 2, 4};
  RunTest(raw1, 6, expected1, 4, 1);

  // Test 2
  int raw2[] = {1, 1, 1, 1};
  int expected2[] = {1};
  RunTest(raw2, 4, expected2, 1, 2);

  // Test 3
  int raw3[] = {};
  int expected3[] = {};
  RunTest(raw3, 0, expected3, 0, 3);

  // Test 4
  int raw4[] = {5, 4, 3, 2, 1};
  int expected4[] = {5, 4, 3, 2, 1};
  RunTest(raw4, 5, expected4, 5, 4);

  return 0;
}
