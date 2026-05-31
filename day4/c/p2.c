#include "p2.h"
#include "base.h"
#include "parser.h"
#include "range.h"

uint64_t p2(const char *input) {
  const char *line = input;
  ParseResult result = {0};
  uint64_t total = 0;

  while (true) {
    result = parse_ranges(line);
    if (!result.ok) break;

    if (range_overlap(&result.first, &result.second)) {
      total++;
    }

    line = result.rest;
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  const char *input = "2-4,6-8\n"
                      "2-3,4-5\n"
                      "5-7,7-9\n"
                      "2-8,3-7\n"
                      "6-6,4-6\n"
                      "2-6,4-8\n";

  assert(p2(input) == 4);
}
#endif
