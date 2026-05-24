#include "p1.h"

uint64_t p1(const char *input) {
  return 123;
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

  assert(p1(input) == 72511);
}
#endif
