#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given an integer array, return 1 if any value appears at least twice,
// and 0 if every element is distinct.
int ContainsDuplicate(IntArr nums) {
  IntArr seen = CreateIntArr();

  for (size_t i = 0; i < nums.length; i++) {
    int found = IASearch(seen, nums.data[i]);
    if (found) {
      return 1;
    }
    seen = IAAppend(seen, nums.data[i]);
  }

  return 0;
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

// free array
void FreeArr(IntArr arr) { free(arr.data); }

// ---------- Tests ----------

void RunTest(int *raw, size_t rawLen, int expected, int testNum) {
  IntArr nums = CreateIntArrFromData(raw, rawLen);

  int result = ContainsDuplicate(nums);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  PrintArr(nums);
  printf("\n");
  printf("Expected: %d\n", expected);
  printf("Result:   %d\n", result);
  printf("PASS: %s\n\n", result == expected ? "YES" : "NO");

  FreeArr(nums);
}

int main() {
  printf("=== Contains Duplicate Tests ===\n\n");

  // Test 1: has duplicate
  int raw1[] = {1, 2, 3, 1};
  RunTest(raw1, 4, 1, 1);

  // Test 2: all distinct
  int raw2[] = {1, 2, 3, 4};
  RunTest(raw2, 4, 0, 2);

  // Test 3: multiple duplicates
  int raw3[] = {1, 1, 1, 3, 3, 4, 3, 2, 4, 2};
  RunTest(raw3, 10, 1, 3);

  // Test 4: single element
  int raw4[] = {7};
  RunTest(raw4, 1, 0, 4);

  // Test 5: two same elements
  int raw5[] = {5, 5};
  RunTest(raw5, 2, 1, 5);

  // Test 6: negative numbers with duplicate
  int raw6[] = {-1, -2, -3, -1};
  RunTest(raw6, 4, 1, 6);

  // Test 7: empty array
  int raw7[] = {};
  RunTest(raw7, 0, 0, 7);

  return 0;
}
