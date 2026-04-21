#include <core/memory.h>
#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>
#include <core/panic.h>
#include <string.h>

// Creates IntArr with capacity of 8,
// Caller is responsible for freeing
IntArr NewIntArr() {
  IntArr intArr;
  usize capacity = 8;
  usize len = 0;
  intArr.data = cmalloc(capacity * sizeof(int));
  intArr.capacity = capacity;
  intArr.len = len;
  return intArr;
}

// Created IntArr with capacity specified with @usize len,
// Caller is responsible for freeing
IntArr NewIntArrFromData(int *data, usize length) {
  IntArr intArr;
  usize capacity = (data == NULL || length == 0) ? 8 : length;

  int *tmp = cmalloc(capacity * sizeof(int));
  if (data != NULL && length > 0) {
    memcpy(tmp, data, (length * sizeof(int)));
    intArr.data = tmp;
    intArr.capacity = capacity;
    intArr.len = length;
  } else {
    intArr.len = 0;
    intArr.data = tmp;
    intArr.capacity = capacity;
  }

  return intArr;
}

IntArr NewIntArrWithCapacity(usize capacity) {
  if (capacity == 0) {
    capacity = INIT_CAPACITY;
  }
  IntArr arr;
  int *tmp = cmalloc(capacity * sizeof(int));
  arr.capacity = capacity;
  arr.len = 0;
  arr.data = tmp;
  return arr;
}

// Appends value at last index of array
void IntArrAppend(IntArr *arr, int value) {
  if (arr->len >= arr->capacity) {
    *arr = IntArrResize(*arr);
  }
  arr->data[arr->len] = value;
  arr->len++;
  return;
}

// Doubles size of old array and copies over data to new array
IntArr IntArrResize(IntArr arr) {
  int *tmp;
  usize newCapacity = arr.capacity * 2;
  tmp = crealloc(arr.data, newCapacity * sizeof(int));
  arr.capacity = newCapacity;
  arr.data = tmp;
  return arr;
}

void IntArrInsert(IntArr *arr, usize index, int value) {
  if (index > arr->len) {
    panic("insert out of bounds error");
  }
  if (arr->len + 1 >= arr->capacity) {
    *arr = IntArrResize(*arr);
  }

  // memmove uses a *ptr -> offset and a bytecount to manipulate buffers
  // shift everything from index onward one slot to the right
  memmove(&arr->data[index + 1], &arr->data[index], (arr->len - index) * sizeof(int));
  arr->data[index] = value;
  arr->len++;
  return;
}

void IntArrDelete(IntArr *arr, usize index) {
  if (index >= arr->len) {
    panic("delete out of bounds error");
  }
  // arr.len - index - 1 "closes" the gap for the old number
  memmove(&arr->data[index], &arr->data[index + 1], (arr->len - index - 1) * sizeof(int));
  arr->len--;
  return;
}

// Returns value at specified index
int IntArrGet(IntArr *arr, usize index) {
  if (index >= arr->len) {
    panic("get arr out of bounds error");
  }
  return arr->data[index];
}

// Sets value at specified index
void IntArrSet(IntArr *arr, usize index, int value) {
  if (index >= arr->len) {
    panic("set arr out of bounds error");
  }
  arr->data[index] = value;
  return;
}

// Returns capacity of array
usize IntArrGetCapacity(IntArr *arr) { return arr->capacity; }

// Returns len of array
usize IntArrGetLen(IntArr *arr) { return arr->len; }

// Decrements len of array by 1 at the last index
void IntArrPop(IntArr *arr) {
  if (arr->len < 1) {
    panic("array is empty");
  }
  arr->len--;
}

// Returns the value at the last index of the array
int IntArrPeek(const IntArr *arr) {
  if (arr->len == 0) {
    panic("arr is empty");
  }
  return arr->data[arr->len - 1];
}

// Returns 1 if value exists in the array or 0 if the value is never found
bool IntArrSearch(const IntArr *arr, int value) {
  for (usize i = 0; i < arr->len; i++) {
    if (arr->data[i] == value) {
      return true;
    }
  }
  return false;
}

IntArr IntArrCpy(IntArr arr) {
  IntArr dupArr;
  int *tmp = cmalloc(arr.len * sizeof(int));
  memcpy(tmp, arr.data, (arr.len * sizeof(int)));
  dupArr.data = tmp;
  dupArr.len = arr.len;
  dupArr.capacity = arr.len;
  return dupArr;
}

void IntArrClr(IntArr *arr) {
  arr->len = 0;
}

void FreeIntArr(IntArr *arr) {
  FREE(arr->data);
}
