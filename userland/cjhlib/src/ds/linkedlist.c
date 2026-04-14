#include <core/memory.h>
#include <ds/linkedlist.h>
#include <stddef.h>
#include <stdlib.h>

LinkedList CreateLinkedList() {
  LinkedList linkedlist;
  linkedlist.head = NULL;
  linkedlist.length = 0;
  return linkedlist;
}

LinkedList CreateLinkedListFromData(int *data, size_t length) {
  LinkedList linkedList = CreateLinkedList();
  for (size_t i = 0; i < length; i++) {
    linkedList = LLAppend(linkedList, data[i]);
  }
  return linkedList;
}

LinkedList LLPrepend(LinkedList list, int value) {
  Node *node = cmalloc(sizeof(Node));
  node->value = value;
  node->next = list.head;
  list.head = node;
  list.length = list.length + 1;
  return list;
}

LinkedList LLAppend(LinkedList list, int value) {
  Node *node = cmalloc(sizeof(Node));
  node->value = value;
  node->next = NULL;
  Node *curr = list.head;

  if (list.head == NULL) {
    list.head = node;
    list.length = list.length + 1;
    return list;
  }

  while (curr->next != NULL) {
    curr = curr->next;
  }

  curr->next = node;
  list.length = list.length + 1;
  return list;
}

LinkedList LLPop(LinkedList list) {
  Node *curr = list.head;

  if (list.head == NULL) {
    exit(1);
  }

  if (list.length == 1) {
    cfree(list.head);
    list.head = NULL;
    list.length = 0;
    return list;
  }

  while (curr->next->next != NULL) {
    curr = curr->next;
  }

  cfree(curr->next);
  curr->next = NULL;
  list.length = list.length - 1;
  return list;
}

void FreeList(LinkedList list) {
  if (list.head == NULL) {
    return;
  }

  if (list.length == 0) {
    return;
  }

  Node *curr = list.head;
  Node *nextNode = NULL;
  while (curr != NULL) {
    nextNode = curr->next;
    cfree(curr);
    curr = nextNode;
  }
}

LinkedList LLRemoveHead(LinkedList list) {
  if (list.head == NULL) {
    exit(1);
  }
  if (list.length == 1) {
    cfree(list.head);
    list.head = NULL;
    list.length = 0;
    return list;
  }
  Node *curr = list.head;
  Node *nextNode = list.head->next;
  cfree(curr);

  list.head = nextNode;
  list.length = list.length - 1;
  return list;
}

LinkedList LLInsertAt(LinkedList list, size_t index, int value) {
  if (list.head == NULL) {
    exit(1);
  }
  if (index >= list.length) {
    exit(1);
  }
  if (index == 0) {
    return LLPrepend(list, value);
  }

  Node *prev = NULL;
  Node *curr = list.head;
  Node *newNode = cmalloc(sizeof(Node));
  newNode->value = value;
  size_t currCount = 0;

  while (currCount != index) {
    prev = curr;
    curr = curr->next;
    currCount++;
  }

  prev->next = newNode;
  newNode->next = curr;

  list.length = list.length + 1;
  return list;
}

LinkedList LLDeleteAt(LinkedList list, size_t index) {
  if (list.head == NULL) {
    exit(1);
  }
  if (index >= list.length) {
    exit(1);
  }
  if (index == 0) {
    return LLRemoveHead(list);
  }

  Node *prev = NULL;
  Node *curr = list.head;
  size_t currCount = 0;

  while (currCount != index) {
    prev = curr;
    curr = curr->next;
    currCount++;
  }

  prev->next = curr->next;
  cfree(curr);

  list.length = list.length - 1;
  return list;
}

int LLSearch(LinkedList list, int value) {
  if (list.head == NULL) {
    return 0;
  }

  Node *curr = list.head;
  while (curr != NULL) {
    if (curr->value == value) {
      return 1;
    } else {
      curr = curr->next;
    }
  }

  return 0;
}

int LLGet(LinkedList list, size_t index) {
  if (list.head == NULL) {
    exit(1);
  }
  if (index >= list.length) {
    exit(1);
  }

  Node *curr = list.head;
  size_t currCount = 0;
  while (currCount != index) {
    curr = curr->next;
    currCount++;
  }
  return curr->value;
}
