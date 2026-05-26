#include "p1.h"

#include "base.h"

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

// Use bitmap to set letters found
// a-z -> 1-26
// A-Z -> 27-52
static uint64_t parse_half(size_t half, const char line[static half]) {
  uint64_t bitmap = 0;

  for (size_t i = 0; i < half; i++) {
    if (islower(line[i])) {
      bitmap |= 1ULL << (line[i] - 'a');
    } else if (isupper(line[i])) {
      bitmap |= 1ULL << (line[i] - 'A' + 26);
    } else {
      expect(false, "Expected alpha, got %c:\n%.*s", line[i], (int)half, line);
    }
  }

  return bitmap;
}

void bitmap_print(uint64_t n) {
  printf("0b");

  for (int i = 63; i >= 0; i--) {
    putchar((n & (1ULL << i)) ? '1' : '0');
    if (i % 8 == 0 && i != 0) putchar('_');
  }

  putchar('\n');
}

uint64_t p1(const char *input) {
  const char *line = input;
  uint64_t total = 0;

  while (*line) {
    size_t delim = strcspn(line, "\n");
    size_t half = delim / 2;

    uint64_t first = parse_half(half, &line[0]);
    uint64_t second = parse_half(half, &line[half]);
    uint64_t common = first & second;

    expect(common != 0, "At least 1 bit set");
    expect((common & (common - 1)) == 0, "Only 1 bit set");

    // Find First (bit) Set - CPU instruction
    // 1-based, 0 if not found
    int priority = ffsll(common);

    total += priority;

    line = line + delim + 1;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  const char *input = "vJrwpWtwJgWrhcsFMMfFFhFp\n"
                      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n"
                      "PmmdzqPrVvPwwTWBwg\n"
                      "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n"
                      "ttgJtRGJQctTZtZT\n"
                      "CrZsJsPPZsGzwwsLwLmpwMDw\n";

  assert(p1(input) == 157);
}
#endif
