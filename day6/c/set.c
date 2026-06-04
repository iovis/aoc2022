#include "set.h"

#include <stdio.h>

static void set_add(uint32_t *self, const char *ptr) {
  *self |= 1u << (*ptr - 'a');
}

void set_init(uint32_t *self, const char *start, size_t n) {
  for (const char *ptr = start; ptr < start + n; ptr++) {
    set_add(self, ptr);
  }
}

size_t set_count(uint32_t self) {
  return stdc_count_ones(self);
}

void set_print(uint32_t n) {
  printf("0b");

  for (int i = 33; i >= 0; i--) {
    putchar((n & (1ULL << i)) ? '1' : '0');
    if (i % 8 == 0 && i != 0) putchar('\'');
  }

  putchar('\n');
}
