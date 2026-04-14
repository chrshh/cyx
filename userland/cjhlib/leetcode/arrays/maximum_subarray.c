#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given an integer array, find the subarray with the largest sum
// and return its sum. A subarray is a contiguous non-empty part
// of the array.
int MaxSubArray(IntArr nums) {
  int res = nums.data[0];
  int curRes = nums.data[0];

  for (size_t i = 1; i < nums.length; i++) {
    if (nums.data[i] > curRes + nums.data[i]) {
      curRes = nums.data[i];
    } else {
      curRes = curRes + nums.data[i];
    }

    if (curRes > res) {
      res = curRes;
    }
  }

  return res;
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

  int result = MaxSubArray(nums);

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
  printf("=== Maximum Subarray Tests ===\n\n");

  // Test 1: classic case, subarray [4,-1,2,1] = 6
  int raw1[] = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
  RunTest(raw1, 9, 6, 1);

  // Test 2: single element
  int raw2[] = {1};
  RunTest(raw2, 1, 1, 2);

  // Test 3: all negative, pick the least negative
  int raw3[] = {-3, -5, -1, -4};
  RunTest(raw3, 4, -1, 3);

  // Test 4: all positive, entire array is the answer
  int raw4[] = {1, 2, 3, 4};
  RunTest(raw4, 4, 10, 4);

  // Test 5: negative then positive
  int raw5[] = {-2, -1, 3, 5};
  RunTest(raw5, 4, 8, 5);

  // Test 6: single negative element
  int raw6[] = {-7};
  RunTest(raw6, 1, -7, 6);

  // Test 7: best subarray is at the end
  int raw7[] = {-1, -2, 5, 6, 7};
  RunTest(raw7, 5, 18, 7);

  return 0;
}
