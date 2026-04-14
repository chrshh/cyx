#include "array_int.h"

typedef struct HashNode {
  struct HashNode *next;
  int key;
  int val;
} HashNode;

// Each HashNode is an array of pointers to linked lists (buckets)
typedef struct {
  HashNode **buckets;
  size_t capacity;
  size_t size;
} HashMap;

HashMap CreateHashMap();
HashMap CreateHashMap128();
HashMap CreateHashMap256();
void FreeHashMap(HashMap hashmap);
HashMap HMResize(HashMap hashmap);

HashMap HMInsert(HashMap map, int key, int value);
int *HMGet(HashMap map, int key);
HashMap HMDelete(HashMap map, int key);

int Hash(int key, size_t capacity);
