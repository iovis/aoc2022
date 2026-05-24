#include "p1.h"

#include "base.h"
#include "parser.h"
#include <stdint.h>

uint64_t p1(const char *input) {
  uint64_t max = 0;
  const char *group = input;

  while (true) {
    ResultParseGroup result = parse_group(group);
    if (!result.ok) break;

    if (result.group > max) max = result.group;
    group = result.rest;
  }

  return max;
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  const char *input = "1000\n"
                      "2000\n"
                      "3000\n"
                      "\n"
                      "4000\n"
                      "\n"
                      "5000\n"
                      "6000\n"
                      "\n"
                      "7000\n"
                      "8000\n"
                      "9000\n"
                      "\n"
                      "10000\n";

  assert(p1(input) == 24000);
}
#endif
