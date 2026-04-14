#include <ds/linkedlist.h>
#include <stdio.h>
#include <stdlib.h>

// ---------- YOU IMPLEMENT THIS ----------
// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list. The list should be made
// by splicing together the nodes of the first two lists.
// Return the merged linked list.
LinkedList MergeTwoLists(LinkedList list1, LinkedList list2) {
  LinkedList result = CreateLinkedList();
  Node *head1 = list1.head;
  Node *head2 = list2.head;

  while (head1 != NULL || head2 != NULL) {
    if (head1 == NULL && head2 == NULL) {
      return result;

    } else if (head1 != NULL && head2 != NULL) {
      if (head1->value == head2->value) {
        result = LLAppend(result, head1->value);
        head1 = head1->next;
        result = LLAppend(result, head2->value);
        head2 = head2->next;
      } else if (head1->value < head2->value) {
        result = LLAppend(result, head1->value);
        head1 = head1->next;
      } else {
        result = LLAppend(result, head2->value);
        head2 = head2->next;
      }
    } else if (head1 != NULL && head2 == NULL) {
      result = LLAppend(result, head1->value);
      head1 = head1->next;
    } else {
      result = LLAppend(result, head2->value);
      head2 = head2->next;
    }
  }
  return result;
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

void RunTest(int *raw1, size_t len1, int *raw2, size_t len2, int *expectedRaw,
             size_t expectedLen, int testNum) {
  LinkedList input1 = CreateLinkedListFromData(raw1, len1);
  LinkedList input2 = CreateLinkedListFromData(raw2, len2);
  LinkedList expected = CreateLinkedListFromData(expectedRaw, expectedLen);

  LinkedList result = MergeTwoLists(input1, input2);

  printf("Test %d:\n", testNum);
  printf("List 1:   ");
  PrintList(CreateLinkedListFromData(raw1, len1));
  printf("\n");
  printf("List 2:   ");
  PrintList(CreateLinkedListFromData(raw2, len2));
  printf("\n");
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
  printf("=== Merge Two Sorted Lists Tests ===\n\n");

  // Test 1: classic case
  int raw1a[] = {1, 2, 4};
  int raw1b[] = {1, 3, 4};
  int exp1[] = {1, 1, 2, 3, 4, 4};
  RunTest(raw1a, 3, raw1b, 3, exp1, 6, 1);

  // Test 2: both empty
  RunTest(NULL, 0, NULL, 0, NULL, 0, 2);

  // Test 3: one empty, one not
  int raw3b[] = {0};
  int exp3[] = {0};
  RunTest(NULL, 0, raw3b, 1, exp3, 1, 3);

  // Test 4: no overlap in values
  int raw4a[] = {1, 3, 5};
  int raw4b[] = {2, 4, 6};
  int exp4[] = {1, 2, 3, 4, 5, 6};
  RunTest(raw4a, 3, raw4b, 3, exp4, 6, 4);

  // Test 5: one list much longer
  int raw5a[] = {1};
  int raw5b[] = {2, 3, 4, 5, 6};
  int exp5[] = {1, 2, 3, 4, 5, 6};
  RunTest(raw5a, 1, raw5b, 5, exp5, 6, 5);

  // Test 6: duplicate values across lists
  int raw6a[] = {1, 1, 1};
  int raw6b[] = {1, 1, 1};
  int exp6[] = {1, 1, 1, 1, 1, 1};
  RunTest(raw6a, 3, raw6b, 3, exp6, 6, 6);

  // Test 7: negative values
  int raw7a[] = {-3, -1, 2};
  int raw7b[] = {-2, 0, 3};
  int exp7[] = {-3, -2, -1, 0, 2, 3};
  RunTest(raw7a, 3, raw7b, 3, exp7, 6, 7);

  return 0;
}
