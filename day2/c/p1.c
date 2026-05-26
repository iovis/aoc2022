#include "p1.h"
#include <stdint.h>

uint64_t p1(const char *input) {
  const char *line = input;
  uint64_t total = 0;

  while (*line) {
    // Rock = 0, Paper = 1, Scissors = 2
    int opponent = line[0] - 'A';
    int me = line[2] - 'X';

    // 0 = draw -> 3 points
    // 1 = win  -> 6 points
    // 2 = lose -> 0 points
    int diff = (me - opponent + 3) % 3;
    int score = (me + 1) + (((diff + 1) % 3) * 3);

    total += score;

    line = &line[4];
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  const char *input = "A Y\n"
                      "B X\n"
                      "C Z\n";

  assert(p1(input) == 15);
}
#endif
