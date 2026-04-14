#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given an array of integers and a target, return the indices of the
// two numbers that add up to target. Return a 2-element IntArr.
// You may assume each input has exactly one solution and you may not
// use the same element twice.
IntArr TwoSum(IntArr nums, int target) {
  IntArr result = CreateIntArr();

  for (size_t i = 0; i < nums.length; i++) {
    for (size_t j = i + 1; j < nums.length; j++) {
      if (nums.data[i] + nums.data[j] == target) {
        result = IAAppend(result, i);
        result = IAAppend(result, j);
        return result;
      }
    }
  }

  return result;
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

// compare arrays (order matters)
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

void RunTest(int *raw, size_t rawLen, int target, int *expectedRaw,
             size_t expectedLen, int testNum) {
  IntArr nums = CreateIntArrFromData(raw, rawLen);
  IntArr expected = CreateIntArrFromData(expectedRaw, expectedLen);

  IntArr result = TwoSum(nums, target);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  PrintArr(nums);
  printf(", target = %d\n", target);
  printf("Expected: ");
  PrintArr(expected);
  printf("\n");
  printf("Result:   ");
  PrintArr(result);
  printf("\n");
  printf("PASS: %s\n\n", ArraysEqual(result, expected) ? "YES" : "NO");

  FreeArr(nums);
  FreeArr(expected);
  FreeArr(result);
}

int main() {
  printf("=== Two Sum Tests ===\n\n");

  // Test 1: [2,7,11,15], target=9 -> [0,1]
  int raw1[] = {2, 7, 11, 15};
  int expected1[] = {0, 1};
  RunTest(raw1, 4, 9, expected1, 2, 1);

  // Test 2: [3,2,4], target=6 -> [1,2]
  int raw2[] = {3, 2, 4};
  int expected2[] = {1, 2};
  RunTest(raw2, 3, 6, expected2, 2, 2);

  // Test 3: [3,3], target=6 -> [0,1]
  int raw3[] = {3, 3};
  int expected3[] = {0, 1};
  RunTest(raw3, 2, 6, expected3, 2, 3);

  // Test 4: [1,5,3,7], target=12 -> [1,3]
  int raw4[] = {1, 5, 3, 7};
  int expected4[] = {1, 3};
  RunTest(raw4, 4, 12, expected4, 2, 4);

  // Test 5: [-1,-2,-3,-4,-5], target=-8 -> [2,4]
  int raw5[] = {-1, -2, -3, -4, -5};
  int expected5[] = {2, 4};
  RunTest(raw5, 5, -8, expected5, 2, 5);

  return 0;
}
