#include <core/memory.h>
#include <ds/hashmap.h>
#include <stdlib.h>

// Default allocation of 64 for decent spacing out amongst ints
HashMap CreateHashMap(void) {
  HashMap hashmap;
  hashmap.buckets = cmalloc(64 * sizeof(HashNode *));
  hashmap.capacity = 64;
  hashmap.size = 0;
  for (size_t i = 0; i < 64; i++) hashmap.buckets[i] = NULL;
  return hashmap;
}

HashMap CreateHashMap128(void) {
  HashMap hashmap;
  hashmap.buckets = cmalloc(128 * sizeof(HashNode *));
  hashmap.capacity = 128;
  hashmap.size = 0;
  for (size_t i = 0; i < 128; i++) hashmap.buckets[i] = NULL;
  return hashmap;
}

HashMap CreateHashMap256(void) {
  HashMap hashmap;
  hashmap.buckets = cmalloc(256 * sizeof(HashNode *));
  hashmap.capacity = 256;
  hashmap.size = 0;
  for (size_t i = 0; i < 256; i++) hashmap.buckets[i] = NULL;
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

// initialize map -> populate -> free
HashMap HMResize(HashMap map) {
  HashMap newMap;
  newMap.capacity = map.capacity * 2;
  newMap.buckets = cmalloc(newMap.capacity * sizeof(HashNode));

  for (size_t i = 0; i < newMap.capacity; i++) {
    newMap.buckets[i] = NULL;
  }
  for (size_t i = 0; i < map.capacity; i++) {
    if (map.buckets[i] == NULL) {
      continue;
    }
    HashNode *curr = map.buckets[i];
    while (map.buckets[i] != NULL) {
      HMInsert(newMap, curr->key, curr->val);
      curr = curr->next;
    }
  }

  FreeHashMap(map);
  return newMap;
}

int HMContains(HashMap map, int key) {
  int lookupKey = Hash(key, map.capacity);
  HashNode *curr = map.buckets[lookupKey];
  while (curr != NULL) {
    if (curr->key == key) {
      return 1;
    }
    curr = curr->next;
  }

  return 0;
}

int Hash(int key, size_t capacity) { return key % capacity; }
