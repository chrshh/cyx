#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

#define ASSERT(condition, msg)                                                 \
  if (condition) {                                                             \
    printf("PASS✅: %s\n", msg);                                               \
  } else {                                                                     \
    printf("FAIL❌: %s\n", msg);                                               \
  }

int main(void) {
  // --- NewIntArr ---
  printf("--- NewIntArr ---\n");
  IntArr arr = NewIntArr();
  ASSERT(arr.capacity == 8, "empty arr created with capacity of 8");
  ASSERT(arr.len == 0, "empty arr created with len of 0");
  FreeIntArr(&arr);

  // --- NewIntArrFromData ---
  printf("--- NewIntArrFromData ---\n");
  int data[5] = {1, 2, 3, 4, 5};
  IntArr dataArr = NewIntArrFromData(data, 5);
  ASSERT(dataArr.capacity == 5, "dataArr created with correct capacity");
  ASSERT(dataArr.len == 5, "dataArr created with correct len");
  ASSERT(dataArr.data[0] == 1, "dataArr first element is correct");
  ASSERT(dataArr.data[4] == 5, "dataArr last element is correct");
  FreeIntArr(&dataArr);

  // --- NewIntArrFromData NULL ---
  printf("--- NewIntArrFromData NULL ---\n");
  IntArr nullArr = NewIntArrFromData(NULL, 0);
  ASSERT(nullArr.capacity == 8, "null data falls back to capacity of 8");
  ASSERT(nullArr.len == 0, "null data arr has len of 0");
  FreeIntArr(&nullArr);

  // --- NewIntArrWithCapacity ---
  printf("--- NewIntArrWithCapacity ---\n");
  IntArr capArr = NewIntArrWithCapacity(16);
  ASSERT(capArr.capacity == 16, "arr created with correct capacity");
  ASSERT(capArr.len == 0, "arr created with len of 0");
  FreeIntArr(&capArr);

  // --- NewIntArrWithCapacity zero falls back ---
  IntArr zeroCapArr = NewIntArrWithCapacity(0);
  ASSERT(zeroCapArr.capacity == INIT_CAPACITY, "zero capacity falls back to INIT_CAPACITY");
  FreeIntArr(&zeroCapArr);

  // --- Append without resize ---
  printf("--- Append Without Resize ---\n");
  int appendData[5] = {1, 2, 3, 4, 5};
  IntArr appendArr = NewIntArrFromData(appendData, 5);
  IntArrAppend(&appendArr, 99);
  ASSERT(appendArr.len == 6, "len incremented after append");
  ASSERT(appendArr.data[5] == 99, "appended value is correct");
  FreeIntArr(&appendArr);

  // --- Append with resize ---
  printf("--- Append With Resize ---\n");
  int resizeData[8] = {1, 2, 3, 4, 5, 6, 7, 8};
  IntArr resizeArr = NewIntArrFromData(resizeData, 8);
  IntArrAppend(&resizeArr, 9);
  ASSERT(resizeArr.capacity == 16, "capacity doubled after resize");
  ASSERT(resizeArr.len == 9, "len incremented after resize append");
  ASSERT(resizeArr.data[8] == 9, "appended value correct after resize");
  ASSERT(resizeArr.data[0] == 1, "existing data preserved after resize");
  FreeIntArr(&resizeArr);

  // --- Get ---
  printf("--- Get ---\n");
  int getData[3] = {10, 20, 30};
  IntArr getArr = NewIntArrFromData(getData, 3);
  ASSERT(IntArrGet(&getArr, 0) == 10, "IntArrGet() first element correct");
  ASSERT(IntArrGet(&getArr, 2) == 30, "IntArrGet() last element correct");
  ASSERT(IntArrGet(&getArr, 1) == 20, "IntArrGet() middle element correct");
  FreeIntArr(&getArr);

  // --- Set ---
  printf("--- Set ---\n");
  int setData[3] = {1, 2, 3};
  IntArr setArr = NewIntArrFromData(setData, 3);
  IntArrSet(&setArr, 0, 100);
  IntArrSet(&setArr, 2, 300);
  ASSERT(setArr.data[0] == 100, "IntArrSet() first element updated");
  ASSERT(setArr.data[2] == 300, "IntArrSet() last element updated");
  ASSERT(setArr.data[1] == 2, "IntArrSet() untouched element unchanged");
  ASSERT(setArr.len == 3, "IntArrSet() does not affect len");
  FreeIntArr(&setArr);

  // --- Pop ---
  printf("--- Pop ---\n");
  int popData[3] = {1, 2, 3};
  IntArr popArr = NewIntArrFromData(popData, 3);
  IntArrPop(&popArr);
  ASSERT(popArr.len == 2, "IntArrPop() decrements len");
  ASSERT(popArr.capacity == 3, "IntArrPop() does not affect capacity");
  FreeIntArr(&popArr);

  // --- Peek ---
  printf("--- Peek ---\n");
  int peekData[3] = {5, 10, 15};
  IntArr peekArr = NewIntArrFromData(peekData, 3);
  ASSERT(IntArrPeek(&peekArr) == 15, "IntArrPeek() returns last element");
  IntArrPop(&peekArr);
  ASSERT(IntArrPeek(&peekArr) == 10, "IntArrPeek() returns correct element after pop");
  FreeIntArr(&peekArr);

  // --- Search ---
  printf("--- Search ---\n");
  int searchData[5] = {3, 6, 9, 12, 15};
  IntArr searchArr = NewIntArrFromData(searchData, 5);
  ASSERT(IntArrSearch(&searchArr, 9) == true, "IntArrSearch() finds existing value");
  ASSERT(IntArrSearch(&searchArr, 3) == true, "IntArrSearch() finds first element");
  ASSERT(IntArrSearch(&searchArr, 15) == true, "IntArrSearch() finds last element");
  ASSERT(IntArrSearch(&searchArr, 99) == false, "IntArrSearch() returns false for missing value");
  FreeIntArr(&searchArr);

  // --- Insert ---
  printf("--- Insert ---\n");
  int insertData[5] = {10, 15, 20, 25, 30};
  IntArr insertArr = NewIntArrFromData(insertData, 5);
  IntArrInsert(&insertArr, 1, 100);
  ASSERT(insertArr.data[1] == 100, "IntArrInsert() value inserted at given index");
  ASSERT(insertArr.len == 6, "IntArrInsert() len incremented by one");
  ASSERT(insertArr.data[0] == 10, "IntArrInsert() elements before index unchanged");
  ASSERT(insertArr.data[2] == 15, "IntArrInsert() elements after index shifted right");
  ASSERT(insertArr.data[5] == 30, "IntArrInsert() last element shifted correctly");
  IntArrInsert(&insertArr, 0, 999);
  ASSERT(insertArr.data[0] == 999, "IntArrInsert() at index 0 correct");
  ASSERT(insertArr.data[1] == 10, "IntArrInsert() at index 0 shifts all elements right");
  IntArrInsert(&insertArr, insertArr.len, 777);
  ASSERT(insertArr.data[insertArr.len - 1] == 777, "IntArrInsert() at last index correct");
  FreeIntArr(&insertArr);

  // --- Delete ---
  printf("--- Delete ---\n");
  int deleteData[5] = {10, 20, 30, 40, 50};
  IntArr deleteArr = NewIntArrFromData(deleteData, 5);
  IntArrDelete(&deleteArr, 2);
  ASSERT(deleteArr.len == 4, "IntArrDelete() len decremented by one");
  ASSERT(deleteArr.data[2] == 40, "IntArrDelete() elements after index shifted left");
  ASSERT(deleteArr.data[0] == 10, "IntArrDelete() elements before index unchanged");
  ASSERT(deleteArr.data[3] == 50, "IntArrDelete() last element shifted correctly");
  IntArrDelete(&deleteArr, 0);
  ASSERT(deleteArr.data[0] == 20, "IntArrDelete() at index 0 shifts all elements left");
  ASSERT(deleteArr.len == 3, "IntArrDelete() at index 0 decrements len");
  IntArrDelete(&deleteArr, deleteArr.len - 1);
  ASSERT(deleteArr.len == 2, "IntArrDelete() last element decrements len");
  FreeIntArr(&deleteArr);

  // --- Copy ---
  printf("--- Copy ---\n");
  int copyData[4] = {1, 2, 3, 4};
  IntArr original = NewIntArrFromData(copyData, 4);
  IntArr copy = IntArrCpy(original);
  ASSERT(copy.len == original.len, "IntArrCpy() copy has same len");
  ASSERT(copy.capacity == original.len, "IntArrCpy() copy capacity matches len");
  ASSERT(copy.data[0] == 1, "IntArrCpy() first element correct");
  ASSERT(copy.data[3] == 4, "IntArrCpy() last element correct");
  ASSERT(copy.data != original.data, "IntArrCpy() copy is a distinct allocation");
  IntArrSet(&copy, 0, 99);
  ASSERT(original.data[0] == 1, "IntArrCpy() mutating copy does not affect original");
  FreeIntArr(&original);
  FreeIntArr(&copy);

  // --- Clear ---
  printf("--- Clear ---\n");
  int clrData[3] = {1, 2, 3};
  IntArr clrArr = NewIntArrFromData(clrData, 3);
  IntArrClr(&clrArr);
  ASSERT(clrArr.len == 0, "IntArrClr() resets len to 0");
  ASSERT(clrArr.capacity == 3, "IntArrClr() does not affect capacity");
  ASSERT(clrArr.data != NULL, "IntArrClr() buffer still allocated");
  FreeIntArr(&clrArr);

  return 0;
}
