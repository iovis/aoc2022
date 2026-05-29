#pragma once

#include <stddef.h>
#include <stdint.h>

uint64_t parse_line(size_t n, const char line[static n]);
void bitmap_print(uint64_t n);

#ifdef TEST
void bitmap_tests(void);
#endif
