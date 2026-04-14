#include <ds/linkedlist.h>
#include <stdio.h>
#include <stdlib.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  printf("=== Linked List Tests ===\n\n");

  // --- CreateList ---
  printf("--- CreateList ---\n");
  LinkedList list = CreateLinkedList();
  ASSERT(list.head == NULL, "new list head is NULL");
  ASSERT(list.length == 0, "new list length is 0");

  // --- Append ---
  printf("--- Append ---\n");
  LinkedList appendList = CreateLinkedList();
  appendList = LLAppend(appendList, 10);
  ASSERT(appendList.length == 1, "length is 1 after first append");
  ASSERT(appendList.head->value == 10, "head value correct after append");
  appendList = LLAppend(appendList, 20);
  appendList = LLAppend(appendList, 30);
  ASSERT(appendList.length == 3, "length is 3 after three appends");
  ASSERT(appendList.head->next->value == 20, "second node value correct");
  ASSERT(appendList.head->next->next->value == 30, "third node value correct");
  FreeList(appendList);

  // --- Prepend ---
  printf("--- Prepend ---\n");
  LinkedList prependList = CreateLinkedList();
  prependList = LLPrepend(prependList, 10);
  ASSERT(prependList.length == 1, "length is 1 after first prepend");
  ASSERT(prependList.head->value == 10, "head value correct after prepend");
  prependList = LLPrepend(prependList, 20);
  ASSERT(prependList.length == 2, "length is 2 after second prepend");
  ASSERT(prependList.head->value == 20, "new head value correct");
  ASSERT(prependList.head->next->value == 10, "old head is now second node");
  FreeList(prependList);

  // --- Get ---
  printf("--- Get ---\n");
  LinkedList getList = CreateLinkedList();
  getList = LLAppend(getList, 10);
  getList = LLAppend(getList, 20);
  getList = LLAppend(getList, 30);
  ASSERT(LLGet(getList, 0) == 10, "LLGet index 0 returns first value");
  ASSERT(LLGet(getList, 1) == 20, "LLGet index 1 returns second value");
  ASSERT(LLGet(getList, 2) == 30, "LLGet index 2 returns third value");
  FreeList(getList);

  // --- Search ---
  printf("--- Search ---\n");
  LinkedList searchList = CreateLinkedList();
  searchList = LLAppend(searchList, 10);
  searchList = LLAppend(searchList, 20);
  searchList = LLAppend(searchList, 30);
  ASSERT(LLSearch(searchList, 10) == 1, "LLSearch finds first element");
  ASSERT(LLSearch(searchList, 30) == 1, "LLSearch finds last element");
  ASSERT(LLSearch(searchList, 99) == 0, "LLSearch returns 0 for missing value");
  FreeList(searchList);

  // --- InsertAt ---
  printf("--- InsertAt ---\n");
  LinkedList insertList = CreateLinkedList();
  insertList = LLAppend(insertList, 10);
  insertList = LLAppend(insertList, 30);
  insertList = LLAppend(insertList, 40);
  insertList = LLInsertAt(insertList, 1, 20);
  ASSERT(insertList.length == 4, "length incremented after LLInsertAt");
  ASSERT(LLGet(insertList, 0) == 10, "elements before index unchanged");
  ASSERT(LLGet(insertList, 1) == 20, "inserted value at correct index");
  ASSERT(LLGet(insertList, 2) == 30, "elements after index shifted");
  insertList = LLInsertAt(insertList, 0, 5);
  ASSERT(insertList.head->value == 5, "InsertAt index 0 becomes new head");
  ASSERT(insertList.length == 5, "length correct after insert at head");
  FreeList(insertList);

  // --- Pop ---
  printf("--- Pop ---\n");
  LinkedList popList = CreateLinkedList();
  popList = LLAppend(popList, 10);
  popList = LLAppend(popList, 20);
  popList = LLAppend(popList, 30);
  popList = LLPop(popList);
  ASSERT(popList.length == 2, "length decremented after LLPop");
  ASSERT(LLGet(popList, 1) == 20, "last element is now 20");
  popList = LLPop(popList);
  popList = LLPop(popList);
  ASSERT(popList.length == 0, "length is 0 after popping all elements");
  ASSERT(popList.head == NULL, "head is NULL after popping all elements");

  // --- RemoveHead ---
  printf("--- RemoveHead ---\n");
  LinkedList removeList = CreateLinkedList();
  removeList = LLAppend(removeList, 10);
  removeList = LLAppend(removeList, 20);
  removeList = LLAppend(removeList, 30);
  removeList = LLRemoveHead(removeList);
  ASSERT(removeList.length == 2, "length decremented after LLRemoveHead");
  ASSERT(removeList.head->value == 20, "new head is second element");
  removeList = LLRemoveHead(removeList);
  removeList = LLRemoveHead(removeList);
  ASSERT(removeList.length == 0, "length is 0 after removing all heads");
  ASSERT(removeList.head == NULL, "head is NULL after removing all heads");

  // --- DeleteAt ---
  printf("--- DeleteAt ---\n");
  LinkedList deleteList = CreateLinkedList();
  deleteList = LLAppend(deleteList, 10);
  deleteList = LLAppend(deleteList, 20);
  deleteList = LLAppend(deleteList, 30);
  deleteList = LLAppend(deleteList, 40);
  deleteList = LLDeleteAt(deleteList, 1);
  ASSERT(deleteList.length == 3, "length decremented after LLDeleteAt");
  ASSERT(LLGet(deleteList, 0) == 10, "elements before index unchanged");
  ASSERT(LLGet(deleteList, 1) == 30, "elements after index shifted left");
  deleteList = LLDeleteAt(deleteList, 0);
  ASSERT(deleteList.head->value == 30, "LLDeleteAt 0 removes head");
  ASSERT(deleteList.length == 2, "length correct after delete at head");
  FreeList(deleteList);

  return 0;
}
