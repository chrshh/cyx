#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main() {
  // --- CreateArr ---
  printf("--- CreateArr ---\n");
  IntArr arr = CreateIntArr();
  ASSERT(arr.capacity == 8, "empty arr created with capacity of 8");
  ASSERT(arr.length == 0, "empty arr created with length of 0");
  free(arr.data);

  // --- CreateArrFromData ---
  printf("--- CreateArrFromData ---\n");
  int data[5] = {1, 2, 3, 4, 5};
  IntArr dataArr = CreateIntArrFromData(data, 5);
  ASSERT(dataArr.capacity == 5, "dataArr created with correct capacity");
  ASSERT(dataArr.length == 5, "dataArr created with correct length");
  ASSERT(dataArr.data[0] == 1, "dataArr first element is correct");
  ASSERT(dataArr.data[4] == 5, "dataArr last element is correct");
  free(dataArr.data);

  // --- CreateArrFromData NULL/empty ---
  printf("--- CreateArrFromData NULL ---\n");
  IntArr nullArr = CreateIntArrFromData(NULL, 0);
  ASSERT(nullArr.capacity == 8, "null data falls back to capacity of 8");
  ASSERT(nullArr.length == 0, "null data arr has length of 0");
  free(nullArr.data);

  // --- Append without resize ---
  printf("--- Append Without Resize ---\n");
  int appendData[5] = {1, 2, 3, 4, 5};
  IntArr appendArr = CreateIntArrFromData(appendData, 5);
  appendArr = IAAppend(appendArr, 99);
  ASSERT(appendArr.length == 6, "length incremented after append");
  ASSERT(appendArr.data[5] == 99, "appended value is correct");
  ASSERT(appendArr.capacity == 5, "capacity unchanged without resize");
  free(appendArr.data);

  // --- Append with resize ---
  printf("--- Append With Resize ---\n");
  int resizeData[8] = {1, 2, 3, 4, 5, 6, 7, 8};
  IntArr resizeArr = CreateIntArrFromData(resizeData, 8);
  resizeArr = IAAppend(resizeArr, 9);
  ASSERT(resizeArr.capacity == 16, "capacity doubled after resize");
  ASSERT(resizeArr.length == 9, "length incremented after resize append");
  ASSERT(resizeArr.data[8] == 9, "appended value correct after resize");
  ASSERT(resizeArr.data[0] == 1, "existing data preserved after resize");
  free(resizeArr.data);

  // --- Get ---
  printf("--- Get ---\n");
  int getData[3] = {10, 20, 30};
  IntArr getArr = CreateIntArrFromData(getData, 3);
  ASSERT(IAGet(getArr, 0) == 10, "IAGet() first element correct");
  ASSERT(IAGet(getArr, 2) == 30, "IAGet() last element correct");
  ASSERT(IAGet(getArr, 1) == 20, "IAGet() middle element correct");
  free(getArr.data);

  // --- Set ---
  printf("--- Set ---\n");
  int setData[3] = {1, 2, 3};
  IntArr setArr = CreateIntArrFromData(setData, 3);
  setArr = IASet(setArr, 0, 100);
  setArr = IASet(setArr, 2, 300);
  ASSERT(setArr.data[0] == 100, "Set() first element updated");
  ASSERT(setArr.data[2] == 300, "Set() last element updated");
  ASSERT(setArr.data[1] == 2, "Set() untouched element unchanged");
  ASSERT(setArr.length == 3, "Set() does not affect length");
  free(setArr.data);

  // --- Pop ---
  printf("--- Pop ---\n");
  int popData[3] = {1, 2, 3};
  IntArr popArr = CreateIntArrFromData(popData, 3);
  popArr = IAPop(popArr);
  ASSERT(popArr.length == 2, "Pop() decrements length");
  ASSERT(popArr.capacity == 3, "Pop() does not affect capacity");
  free(popArr.data);

  // --- Peek ---
  printf("--- Peek ---\n");
  int peekData[3] = {5, 10, 15};
  IntArr peekArr = CreateIntArrFromData(peekData, 3);
  ASSERT(IAPeek(peekArr) == 15, "IAPeek() returns last element");
  peekArr = IAPop(peekArr);
  ASSERT(IAPeek(peekArr) == 10, "IAPeek() returns correct element after pop");
  free(peekArr.data);

  // --- Search ---
  printf("--- Search ---\n");
  int searchData[5] = {3, 6, 9, 12, 15};
  IntArr searchArr = CreateIntArrFromData(searchData, 5);
  ASSERT(IASearch(searchArr, 9) == 1, "IASearch() finds existing value");
  ASSERT(IASearch(searchArr, 3) == 1, "IASearch() finds first element");
  ASSERT(IASearch(searchArr, 15) == 1, "IASearch() finds last element");
  ASSERT(IASearch(searchArr, 99) == 0, "IASearch() returns 0 for missing value");
  free(searchArr.data);

  // --- Insert ---
  printf("--- Insert ---\n");
  int insertData[5] = {10, 15, 20, 25, 30};
  IntArr insertArr = CreateIntArrFromData(insertData, 5);
  insertArr = IAInsert(insertArr, 1, 100);
  ASSERT(insertArr.data[1] == 100, "Insert() value inserted at given index");
  ASSERT(insertArr.length == 6, "Insert() length incremented by one");
  ASSERT(insertArr.data[0] == 10, "Insert() elements before index unchanged");
  ASSERT(insertArr.data[2] == 15,
         "Insert() elements after index shifted right");
  ASSERT(insertArr.data[5] == 30, "Insert() last element shifted correctly");
  insertArr = IAInsert(insertArr, 0, 999);
  ASSERT(insertArr.data[0] == 999, "Insert() at index 0 correct");
  ASSERT(insertArr.data[1] == 10,
         "Insert() at index 0 shifts all elements right");
  insertArr = IAInsert(insertArr, insertArr.length, 777);
  ASSERT(insertArr.data[insertArr.length - 1] == 777,
         "Insert() at last index correct");
  free(insertArr.data);

  // --- Delete ---
  printf("--- Delete ---\n");
  int deleteData[5] = {10, 20, 30, 40, 50};
  IntArr deleteArr = CreateIntArrFromData(deleteData, 5);
  deleteArr = IADelete(deleteArr, 2);
  ASSERT(deleteArr.length == 4, "Delete() length decremented by one");
  ASSERT(deleteArr.data[2] == 40, "Delete() elements after index shifted left");
  ASSERT(deleteArr.data[0] == 10, "Delete() elements before index unchanged");
  ASSERT(deleteArr.data[3] == 50, "Delete() last element shifted correctly");
  deleteArr = IADelete(deleteArr, 0);
  ASSERT(deleteArr.data[0] == 20,
         "Delete() at index 0 shifts all elements left");
  ASSERT(deleteArr.length == 3, "Delete() at index 0 decrements length");
  deleteArr = IADelete(deleteArr, deleteArr.length - 1);
  ASSERT(deleteArr.data[deleteArr.length - 1] != 50,
         "Delete() last element removed");
  ASSERT(deleteArr.length == 2, "Delete() last element decrements length");
  free(deleteArr.data);

  return 0;
}
