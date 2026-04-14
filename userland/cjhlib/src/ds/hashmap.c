#include <core/memory.h>
#include <ds/hashmap.h>
#include <stdlib.h>

// Default allocation of 64 for decent spacing out amongst ints
HashMap CreateHashMap() {
  HashMap hashmap;
  hashmap.buckets = cmalloc(64 * sizeof(HashNode));
  hashmap.capacity = 64;
  return hashmap;
}

HashMap CreateHashMap128() {
  HashMap hashmap;
  hashmap.buckets = cmalloc(128 * sizeof(HashNode));
  hashmap.capacity = 128;
  return hashmap;
}

HashMap CreateHashMap256() {
  HashMap hashmap;
  hashmap.buckets = cmalloc(256 * sizeof(HashNode));
  hashmap.capacity = 256;
  return hashmap;
}

void FreeHashMap(HashMap hashmap) {
  cfree(hashmap.buckets);
  hashmap.buckets = NULL;
  hashmap.size = 0;
}

HashMap HMInsert(HashMap map, int key, int val) {
  if (val < 0) {
    val *= -1;
  }
  if ((float)map.size / map.capacity > 0.75) {
    map = HMResize(map);
  }
  HashNode *node = cmalloc(sizeof(HashNode));

  // This converts the users key -> bucket[index]
  int lookupKey = Hash(key, map.capacity);
  node->key = key;
  node->val = val;
  node->next = map.buckets[lookupKey];
  map.buckets[lookupKey] = node;
  map.size += 1;

  return map;
}

int *HMGet(HashMap map, int key) {
  // If key exists, traverse linked list nodes
  // Otherwise return NULL
  int lookupKey = Hash(key, map.capacity);
  HashNode *curr = map.buckets[lookupKey];

  while (curr != NULL) {
    if (curr->key == key) {
      return &curr->val;
    } else {
      curr = curr->next;
    }
  }
  return NULL;
}

HashMap HMDelete(HashMap map, int key) {
  int lookupKey = Hash(key, map.capacity);
  HashNode *prev = NULL;
  HashNode *curr = map.buckets[lookupKey];

  while (curr != NULL && curr->key != key) {
    prev = curr;
    curr = curr->next;
  }
  if (!curr)
    return map;
  if (!prev) {
    map.buckets[lookupKey] = curr->next;
  } else {
    prev->next = curr->next;
  }

  map.size -= 1;
  cfree(curr);
  return map;
}

HashMap HMResize(HashMap map) { return map; }

int Hash(int key, size_t capacity) { return key % capacity; }
