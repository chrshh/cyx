#include <ds/linkedlist.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given the head of a linked list, remove the nth node from
// the end of the list and return the modified list.
// n is guaranteed to be valid (1 <= n <= list.length).
// Example: [1,2,3,4,5] n=2 -> [1,2,3,5]
LinkedList RemoveNthFromEnd(LinkedList list, int n) {
  return LLDeleteAt(list, list.length - n);
}

// ---------- Utilities ----------

void PrintList(LinkedList list) {
  printf("[");
  Node *curr = list.head;
  while (curr != NULL) {
    printf("%d", curr->value);
    if (curr->next != NULL)
      printf(", ");
    curr = curr->next;
  }
  printf("]");
}

int ListsEqual(LinkedList a, LinkedList b) {
  if (a.length != b.length)
    return 0;
  Node *ca = a.head;
  Node *cb = b.head;
  while (ca != NULL && cb != NULL) {
    if (ca->value != cb->value)
      return 0;
    ca = ca->next;
    cb = cb->next;
  }
  return 1;
}

// ---------- Tests ----------

void RunTest(int *raw, size_t rawLen, int n, int *expectedRaw,
             size_t expectedLen, int testNum) {
  LinkedList input = CreateLinkedListFromData(raw, rawLen);
  LinkedList expected = CreateLinkedListFromData(expectedRaw, expectedLen);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  PrintList(input);
  printf(", n = %d\n", n);

  LinkedList result = RemoveNthFromEnd(input, n);

  printf("Expected: ");
  PrintList(expected);
  printf("\n");
  printf("Result:   ");
  PrintList(result);
  printf("\n");
  printf("PASS: %s\n\n", ListsEqual(result, expected) ? "YES" : "NO");

  FreeList(expected);
  FreeList(result);
}

int main() {
  printf("=== Remove Nth Node From End of List Tests ===\n\n");

  // Test 1: classic case, remove 2nd from end
  int raw1[] = {1, 2, 3, 4, 5};
  int exp1[] = {1, 2, 3, 5};
  RunTest(raw1, 5, 2, exp1, 4, 1);

  // Test 2: single element, remove it
  int raw2[] = {1};
  RunTest(raw2, 1, 1, NULL, 0, 2);

  // Test 3: two elements, remove last
  int raw3[] = {1, 2};
  int exp3[] = {1};
  RunTest(raw3, 2, 1, exp3, 1, 3);

  // Test 4: two elements, remove first (head)
  int raw4[] = {1, 2};
  int exp4[] = {2};
  RunTest(raw4, 2, 2, exp4, 1, 4);

  // Test 5: remove the last node
  int raw5[] = {1, 2, 3};
  int exp5[] = {1, 2};
  RunTest(raw5, 3, 1, exp5, 2, 5);

  // Test 6: remove the head (nth from end == length)
  int raw6[] = {1, 2, 3, 4};
  int exp6[] = {2, 3, 4};
  RunTest(raw6, 4, 4, exp6, 3, 6);

  // Test 7: remove middle node
  int raw7[] = {10, 20, 30, 40, 50};
  int exp7[] = {10, 20, 40, 50};
  RunTest(raw7, 5, 3, exp7, 4, 7);

  return 0;
}
