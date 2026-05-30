#pragma once

#include <stdint.h>

typedef struct {
  uint64_t start;
  uint64_t end;
} Range;

bool range_fully_contained(const Range *a, const Range *b);

#ifdef TEST
void range_tests(void);
#endif
