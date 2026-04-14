#include <stddef.h>

typedef struct Node {
  int value;
  struct Node *next;
} Node;

typedef struct {
  Node *head;
  size_t length;
} LinkedList;

LinkedList CreateLinkedList();
LinkedList CreateLinkedListFromData(int *data, size_t length);

LinkedList LLPrepend(LinkedList linkedlist, int value);
LinkedList LLAppend(LinkedList linkedlist, int value);
LinkedList LLInsertAt(LinkedList linkedlist, size_t index, int value);

LinkedList LLPop(LinkedList linkedlist);
LinkedList LLRemoveHead(LinkedList linkedlist);
LinkedList LLDeleteAt(LinkedList, size_t index);

int LLSearch(LinkedList linkedlist, int value);
int LLGet(LinkedList list, size_t index);

void FreeList(LinkedList linkedlist);
