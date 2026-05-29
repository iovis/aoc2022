#include "p2.h"

#include "base.h"
#include "bitmap.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

uint64_t p2(const char *input) {
  const char *line = input;
  uint64_t total = 0;

  while (*line) {
    uint64_t priorities = ~0; // all 1s

    for (int i = 0; i < 3; i++) {
      expect(*line, "we haven't run out of groups");
      size_t delim = strcspn(line, "\n");
      priorities &= parse_line(delim, &line[0]);
      line = line + delim + 1;
    }

    expect(priorities != 0, "At least 1 bit set");
    expect((priorities & (priorities - 1)) == 0, "Only 1 bit set");

    // Find First (bit) Set - CPU instruction
    // 1-based, 0 if not found
    int priority = ffsll(priorities);

    total += priority;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  const char *input = "vJrwpWtwJgWrhcsFMMfFFhFp\n"
                      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n"
                      "PmmdzqPrVvPwwTWBwg\n"
                      "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n"
                      "ttgJtRGJQctTZtZT\n"
                      "CrZsJsPPZsGzwwsLwLmpwMDw\n";

  assert(p2(input) == 70);
}
#endif
