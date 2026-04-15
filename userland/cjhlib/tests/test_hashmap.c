#include <ds/hashmap.h>
#include <stdio.h>
#include <stdlib.h>


#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  printf("=== HashMap Tests ===\n\n");

  // --- CreateHashMap ---
  printf("--- CreateHashMap ---\n");
  HashMap map = CreateHashMap();
  ASSERT(map.capacity == 64, "default map created with capacity of 64");
  ASSERT(map.size == 0, "default map created with size of 0");
  FreeHashMap(map);

  // --- CreateHashMap128 ---
  printf("--- CreateHashMap128 ---\n");
  HashMap map128 = CreateHashMap128();
  ASSERT(map128.capacity == 128, "map128 created with capacity of 128");
  ASSERT(map128.size == 0, "map128 created with size of 0");
  FreeHashMap(map128);

  // --- CreateHashMap256 ---
  printf("--- CreateHashMap256 ---\n");
  HashMap map256 = CreateHashMap256();
  ASSERT(map256.capacity == 256, "map256 created with capacity of 256");
  ASSERT(map256.size == 0, "map256 created with size of 0");
  FreeHashMap(map256);

  // --- HMInsert ---
  printf("--- HMInsert ---\n");
  HashMap insertMap = CreateHashMap();
  insertMap = HMInsert(insertMap, 1, 100);
  ASSERT(insertMap.size == 1, "size incremented to 1 after first insert");
  insertMap = HMInsert(insertMap, 2, 200);
  insertMap = HMInsert(insertMap, 3, 300);
  ASSERT(insertMap.size == 3, "size is 3 after three inserts");

  // HMInsert converts negative values to positive (abs)
  insertMap = HMInsert(insertMap, 4, -50);
  int *negVal = HMGet(insertMap, 4);
  ASSERT(negVal != NULL, "key with negative value was inserted");
  ASSERT(*negVal == 50, "negative value stored as absolute value");
  FreeHashMap(insertMap);

  // --- HMGet ---
  printf("--- HMGet ---\n");
  HashMap getMap = CreateHashMap();
  getMap = HMInsert(getMap, 10, 100);
  getMap = HMInsert(getMap, 20, 200);
  getMap = HMInsert(getMap, 30, 300);

  int *val = HMGet(getMap, 10);
  ASSERT(val != NULL, "HMGet returns non-NULL for existing key");
  ASSERT(*val == 100, "HMGet returns correct value for key 10");

  int *val2 = HMGet(getMap, 20);
  ASSERT(val2 != NULL && *val2 == 200, "HMGet returns correct value for key 20");

  int *val3 = HMGet(getMap, 30);
  ASSERT(val3 != NULL && *val3 == 300, "HMGet returns correct value for key 30");

  int *missing = HMGet(getMap, 999);
  ASSERT(missing == NULL, "HMGet returns NULL for missing key");
  FreeHashMap(getMap);

  // --- HMDelete ---
  printf("--- HMDelete ---\n");
  HashMap deleteMap = CreateHashMap();
  deleteMap = HMInsert(deleteMap, 1, 10);
  deleteMap = HMInsert(deleteMap, 2, 20);
  deleteMap = HMInsert(deleteMap, 3, 30);

  deleteMap = HMDelete(deleteMap, 2);
  ASSERT(deleteMap.size == 2, "size decremented after delete");
  ASSERT(HMGet(deleteMap, 2) == NULL, "deleted key no longer retrievable");
  ASSERT(HMGet(deleteMap, 1) != NULL, "non-deleted key still present");
  ASSERT(HMGet(deleteMap, 3) != NULL, "non-deleted key still present");

  // Delete non-existent key (should be a no-op)
  deleteMap = HMDelete(deleteMap, 999);
  ASSERT(deleteMap.size == 2, "size unchanged after deleting missing key");

  // Delete remaining keys
  deleteMap = HMDelete(deleteMap, 1);
  deleteMap = HMDelete(deleteMap, 3);
  ASSERT(deleteMap.size == 0, "size is 0 after deleting all keys");
  FreeHashMap(deleteMap);

  // --- HMContains ---
  printf("--- HMContains ---\n");
  HashMap containsMap = CreateHashMap();
  containsMap = HMInsert(containsMap, 5, 50);
  containsMap = HMInsert(containsMap, 10, 100);

  ASSERT(HMContains(containsMap, 5) == 1, "Contains returns 1 for existing key 5");
  ASSERT(HMContains(containsMap, 10) == 1, "Contains returns 1 for existing key 10");
  ASSERT(HMContains(containsMap, 99) == 0, "Contains returns 0 for missing key");

  containsMap = HMDelete(containsMap, 5);
  ASSERT(HMContains(containsMap, 5) == 0, "Contains returns 0 after key is deleted");
  FreeHashMap(containsMap);

  // --- Hash ---
  printf("--- Hash ---\n");
  ASSERT(Hash(0, 64) == 0, "Hash of 0 with capacity 64 is 0");
  ASSERT(Hash(64, 64) == 0, "Hash of 64 with capacity 64 wraps to 0");
  ASSERT(Hash(65, 64) == 1, "Hash of 65 with capacity 64 is 1");
  ASSERT(Hash(7, 64) == 7, "Hash of 7 with capacity 64 is 7");

  // --- Collision handling ---
  printf("--- Collision Handling ---\n");
  HashMap collMap = CreateHashMap();
  // Keys 0 and 64 both hash to bucket 0 with capacity 64
  collMap = HMInsert(collMap, 0, 10);
  collMap = HMInsert(collMap, 64, 20);
  ASSERT(collMap.size == 2, "two colliding keys both inserted");
  int *c1 = HMGet(collMap, 0);
  int *c2 = HMGet(collMap, 64);
  ASSERT(c1 != NULL && *c1 == 10, "first colliding key returns correct value");
  ASSERT(c2 != NULL && *c2 == 20, "second colliding key returns correct value");
  collMap = HMDelete(collMap, 0);
  ASSERT(HMGet(collMap, 0) == NULL, "first colliding key deleted correctly");
  ASSERT(HMGet(collMap, 64) != NULL, "second colliding key unaffected by delete");
  FreeHashMap(collMap);

  return 0;
}
