#include "range.h"

bool range_fully_contained(const Range *a, const Range *b) {
  return (a->start >= b->start && a->end <= b->end) || (b->start >= a->start && b->end <= a->end);
}

#ifdef TEST
#include <assert.h>

static void range_fully_contained_test(void) {
  Range a, b;

  a = (Range){.start = 2, .end = 4};
  b = (Range){.start = 6, .end = 8};
  assert(range_fully_contained(&a, &b) == false);

  a = (Range){.start = 5, .end = 7};
  b = (Range){.start = 7, .end = 9};
  assert(range_fully_contained(&a, &b) == false);

  a = (Range){.start = 2, .end = 8};
  b = (Range){.start = 3, .end = 7};
  assert(range_fully_contained(&a, &b) == true);

  a = (Range){.start = 6, .end = 6};
  b = (Range){.start = 4, .end = 6};
  assert(range_fully_contained(&a, &b) == true);
}

void range_tests(void) {
  range_fully_contained_test();
}
#endif
