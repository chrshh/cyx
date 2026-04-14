#include <ds/linkedlist.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// Given the head of a singly linked list, reverse the list,
// and return the reversed list.
// Example: 1 -> 2 -> 3 -> 4 -> 5 becomes 5 -> 4 -> 3 -> 2 -> 1
LinkedList ReverseList(LinkedList list) {
  Node *node = NULL;
  size_t i = 0;

  while (i < list.length) {
    Node *tmp = list.head->next;
    list.head->next = node;
    node = list.head;
    list.head = tmp;
    i++;
  }
  list.head = node;
  return list;
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

void RunTest(int *raw, size_t rawLen, int *expectedRaw, size_t expectedLen,
             int testNum) {
  LinkedList input = CreateLinkedListFromData(raw, rawLen);
  LinkedList expected = CreateLinkedListFromData(expectedRaw, expectedLen);

  LinkedList result = ReverseList(input);

  printf("Test %d:\n", testNum);
  printf("Input:    ");
  LinkedList display = CreateLinkedListFromData(raw, rawLen);
  PrintList(display);
  printf("\n");
  printf("Expected: ");
  PrintList(expected);
  printf("\n");
  printf("Result:   ");
  PrintList(result);
  printf("\n");
  printf("PASS: %s\n\n", ListsEqual(result, expected) ? "YES" : "NO");

  FreeList(display);
  FreeList(expected);
  FreeList(result);
}

int main() {
  printf("=== Reverse Linked List Tests ===\n\n");

  // Test 1: standard case
  int raw1[] = {1, 2, 3, 4, 5};
  int exp1[] = {5, 4, 3, 2, 1};
  RunTest(raw1, 5, exp1, 5, 1);

  // Test 2: two elements
  int raw2[] = {1, 2};
  int exp2[] = {2, 1};
  RunTest(raw2, 2, exp2, 2, 2);

  // Test 3: single element
  int raw3[] = {1};
  int exp3[] = {1};
  RunTest(raw3, 1, exp3, 1, 3);

  // Test 4: three elements
  int raw4[] = {10, 20, 30};
  int exp4[] = {30, 20, 10};
  RunTest(raw4, 3, exp4, 3, 4);

  // Test 5: negative values
  int raw5[] = {-1, -2, -3};
  int exp5[] = {-3, -2, -1};
  RunTest(raw5, 3, exp5, 3, 5);

  // Test 6: mixed positive and negative
  int raw6[] = {3, -1, 4, -1, 5};
  int exp6[] = {5, -1, 4, -1, 3};
  RunTest(raw6, 5, exp6, 5, 6);

  return 0;
}
