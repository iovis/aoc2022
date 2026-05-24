#include "p2.h"

#include "base.h"
#include "parser.h"
#include "stb_ds.h"
#include <stddefer.h>
#include <stdint.h>
#include <stdlib.h>

// Sort in reverse order
static int u64cmp_reverse(const void *a, const void *b) {
  const uint64_t *aa = a;
  const uint64_t *bb = b;

  return (*aa < *bb) - (*aa > *bb);
}

uint64_t p2(const char *input) {
  uint64_t *groups = nullptr;
  const char *group = input;
  defer arrfree(groups);

  while (true) {
    ResultParseGroup result = parse_group(group);
    if (!result.ok) break;

    arrpush(groups, result.group);

    group = result.rest;
  }

  qsort(groups, arrlen(groups), sizeof(*groups), u64cmp_reverse);

  expect(arrlen(groups) >= 3);

  uint64_t total = 0;
  for (int i = 0; i < 3; i++) {
    total += groups[i];
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
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

  assert(p2(input) == 45000);
}
#endif
