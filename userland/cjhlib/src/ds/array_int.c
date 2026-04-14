#include <core/memory.h>
#include <ds/array_int.h>
#include <stdio.h>
#include <stdlib.h>

// Creates IntArr with capacity of 8,
// Caller is responsible for freeing
IntArr CreateIntArr() {
  IntArr intArr;
  size_t capacity = 8;
  size_t len = 0;
  intArr.data = cmalloc(capacity * sizeof(int));
  intArr.capacity = capacity;
  intArr.length = len;
  return intArr;
}

// Created IntArr with capacity specified with @size_t length,
// Caller is responsible for freeing
IntArr CreateIntArrFromData(int *data, size_t length) {
  IntArr intArr;
  size_t capacity = (data == NULL || length == 0) ? 8 : length;
  int *tmp = cmalloc(capacity * sizeof(int));
  size_t len = 0;
  if (data != NULL && length > 0) {
    len = length;
    for (size_t i = 0; i < len; i++) {
      tmp[i] = data[i];
    }
  }
  intArr.data = tmp;
  intArr.capacity = capacity;
  intArr.length = len;
  return intArr;
}

IntArr CreateIntArrWithCapacity(size_t capacity) {
  IntArr arr;
  int *tmp = cmalloc(capacity * sizeof(int));
  arr.capacity = capacity;
  arr.length = 0;
  arr.data = tmp;
  return arr;
}

// Appends value at last index of array
IntArr IAAppend(IntArr arr, int value) {
  if (arr.length >= arr.capacity) {
    arr = IAResize(arr);
  }
  arr.data[arr.length] = value;
  arr.length++;
  return arr;
}

// Doubles size of old array and copies over data to new array
IntArr IAResize(IntArr arr) {
  int *tmp;
  size_t newCapacity = arr.capacity * 2;
  tmp = realloc(arr.data, newCapacity * sizeof(int));
  if (!tmp) {
    fprintf(stderr, "Resize: realloc failed\n");
    exit(1);
  }
  arr.capacity = newCapacity;
  arr.data = tmp;
  return arr;
}

IntArr IAInsert(IntArr arr, size_t index, int value) {
  if (index > arr.length) {
    fprintf(stderr, "Insert: insert out of bounds\n");
    exit(1);
  }

  if (arr.length + 1 >= arr.capacity) {
    arr = IAResize(arr);
  }

  int *tmp = cmalloc(arr.capacity * sizeof(int));

  for (size_t i = 0; i < arr.length; i++) {
    if (i < index) {
      tmp[i] = arr.data[i];
    } else {
      tmp[i + 1] = arr.data[i];
    }
  }

  tmp[index] = value;

  cfree(arr.data);
  arr.length = arr.length + 1;
  arr.data = tmp;
  return arr;
}

IntArr IADelete(IntArr arr, size_t index) {
  if (index >= arr.length) {
    fprintf(stderr, "Delete: out of bounds\n");
    exit(1);
  }

  int *tmp = cmalloc(arr.capacity * sizeof(int));

  for (size_t i = 0; i < arr.length; i++) {
    if (i < index) {
      tmp[i] = arr.data[i];
    } else {
      tmp[i] = arr.data[i + 1];
    }
  }

  cfree(arr.data);
  arr.length = arr.length - 1;
  arr.data = tmp;
  return arr;
}

// Returns value at specified index
int IAGet(IntArr arr, int index) {
  if (index < 0 || index >= (int)arr.length) {
    fprintf(stderr, "Get: index %d out of bounds (length: %zu)\n", index,
            arr.length);
    exit(1);
  }
  return arr.data[index];
}

// Sets value at specified index
IntArr IASet(IntArr arr, int index, int value) {
  if (index < 0 || index >= (int)arr.length) {
    fprintf(stderr, "Set: index %d out of bounds (length: %zu)\n", index,
            arr.length);
    exit(1);
  }
  arr.data[index] = value;
  return arr;
}

// Returns capacity of array
size_t IAGetCapacity(IntArr arr) { return arr.capacity; }

// Returns length of array
size_t IAGetLen(IntArr arr) { return arr.length; }

// Decrements length of array by 1 at the last index
IntArr IAPop(IntArr arr) {
  if (arr.length < 1) {
    fprintf(stderr, "Pop: array is empty\n");
    exit(1);
  }
  arr.length--;
  return arr;
}

// Returns the value at the last index of the array
int IAPeek(IntArr arr) {
  if (arr.length == 0) {
    fprintf(stderr, "Peek: array is empty\n");
    exit(1);
  }
  return arr.data[arr.length - 1];
}

// Returns 1 if value exists in the array or 0 if the value is never found
int IASearch(IntArr arr, int value) {
  for (size_t i = 0; i < arr.length; i++) {
    if (arr.data[i] == value) {
      return 1;
    }
  }
  return 0;
}

void FreeIntArr(IntArr arr) { cfree(arr.data); }
