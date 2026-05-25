#include "p2.h"

#include "base.h"
#include "parser.h"
#include <stdint.h>
#include <string.h>

static void insert_if_top(size_t len, uint64_t groups[static len], uint64_t group) {
  for (size_t i = 0; i < len; i++) {
    if (group > groups[i]) {
      if (i < len - 1) {
        // shift the rest of the elements right by 1
        memmove(&groups[i + 1], &groups[i], (len - (i + 1)) * sizeof(groups[0]));
      }

      groups[i] = group;
      return;
    }
  }
}

uint64_t p2(const char *input) {
  uint64_t groups[3] = {0};
  const char *group = input;

  while (true) {
    ResultParseGroup result = parse_group(group);
    if (!result.ok) break;

    insert_if_top(3, groups, result.group);

    group = result.rest;
  }

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
