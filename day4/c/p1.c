#include "p1.h"

uint64_t p1(const char *input) {
  (void)input;
  return 0;
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  const char *input = "2-4,6-8\n"
                      "2-3,4-5\n"
                      "5-7,7-9\n"
                      "2-8,3-7\n"
                      "6-6,4-6\n"
                      "2-6,4-8\n";

  assert(p1(input) == 2);
}
#endif
