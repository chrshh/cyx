#include <stddef.h>

// must maintain capacity >= length !!
typedef struct {
  int *data;
  size_t length;
  size_t capacity;
} IntArr;

// Initialization
IntArr CreateIntArr();
IntArr CreateIntArrFromData(int *data, size_t length);
IntArr CreateIntArrWithCapacity(size_t capacity);

IntArr IAAppend(IntArr arr, int value);
IntArr IAResize(IntArr arr);
IntArr IAInsert(IntArr arr, size_t index, int value);
IntArr IADelete(IntArr arr, size_t index);

int IAGet(IntArr arr, int index);
IntArr IASet(IntArr arr, int index, int value);
IntArr IAPop(IntArr arr);

int IAPeek(IntArr arr);
int IASearch(IntArr arr, int value);

size_t IAGetCapacity(IntArr arr);
size_t IAGetLen(IntArr arr);
