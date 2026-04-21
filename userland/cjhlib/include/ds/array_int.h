#include "core/types.h"
#include <stdbool.h>
#include <stddef.h>

#define INIT_CAPACITY 8

// must maintain capacity >= length !!
typedef struct {
  int *data;
  usize len;
  usize capacity;
} IntArr;

// Initialization
IntArr NewIntArr(void);
IntArr NewIntArrFromData(int *data, usize length);
IntArr NewIntArrWithCapacity(usize capacity);
void FreeIntArr(IntArr *arr);

void IntArrAppend(IntArr *arr, int value);
IntArr IntArrResize(IntArr arr);
void IntArrInsert(IntArr *arr, usize index, int value);
void IntArrDelete(IntArr *arr, usize index);

int IntArrGet(IntArr *arr, usize index);
void IntArrSet(IntArr *arr, usize index, int value);
void IntArrPop(IntArr *arr);

int IntArrPeek(const IntArr *arr);
bool IntArrSearch(const IntArr *arr, int value);

usize IntArrGetCapacity(IntArr *arr);
usize IntArrGetLen(IntArr *arr);

IntArr IntArrSlice(IntArr, usize start, usize end);
IntArr IntArrCpy(IntArr arr);
IntArr IntArrConcat(IntArr arr1, IntArr arr2);
void IntArrClr(IntArr *arr);
