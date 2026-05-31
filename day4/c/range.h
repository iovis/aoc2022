#pragma once

#include <stdint.h>

typedef struct {
  uint64_t start;
  uint64_t end;
} Range;

bool range_fully_contained(const Range *a, const Range *b);
bool range_contains(const Range *a, uint64_t n);
bool range_overlap(const Range *a, const Range *b);

#ifdef TEST
void range_tests(void);
#endif
