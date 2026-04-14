#include <ds/array_int.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given an array of stock prices where prices[i] is the price on day i,
// find the maximum profit from one buy and one sell (buy before sell).
// Return 0 if no profit is possible.
int MaxProfit(IntArr prices) {
  int maxProfit = 0;
  size_t i = 0;
  size_t j = 1;

  while (j < prices.length) {
    int profit = prices.data[j] - prices.data[i];

    if (profit < 0) {
      i = j;
      j++;
    } else {
      if (profit > maxProfit) {
        maxProfit = profit;
      }
      j++;
    }
  }

  return maxProfit;
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
  IntArr prices = CreateIntArrFromData(raw, rawLen);

  int result = MaxProfit(prices);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  PrintArr(prices);
  printf("\n");
  printf("Expected: %d\n", expected);
  printf("Result:   %d\n", result);
  printf("PASS: %s\n\n", result == expected ? "YES" : "NO");

  FreeArr(prices);
}

int main() {
  printf("=== Best Time to Buy and Sell Stock Tests ===\n\n");

  // Test 1: classic case, buy day 1 sell day 4 -> profit 5
  int raw1[] = {7, 1, 5, 3, 6, 4};
  RunTest(raw1, 6, 5, 1);

  // Test 2: prices only go down, no profit possible
  int raw2[] = {7, 6, 4, 3, 1};
  RunTest(raw2, 5, 0, 2);

  // Test 3: buy first day sell last day
  int raw3[] = {1, 2, 3, 4, 5};
  RunTest(raw3, 5, 4, 3);

  // Test 4: single element, can't trade
  int raw4[] = {5};
  RunTest(raw4, 1, 0, 4);

  // Test 5: best buy is not the first element
  int raw5[] = {3, 8, 2, 10, 1};
  RunTest(raw5, 5, 8, 5);

  // Test 6: two elements profit
  int raw6[] = {1, 4};
  RunTest(raw6, 2, 3, 6);

  return 0;
}
