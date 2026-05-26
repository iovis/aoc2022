#include "p2.h"
#include <stdint.h>

uint64_t p2(const char *input) {
  const char *line = input;
  uint64_t total = 0;

  while (*line) {
    // hand: Rock = 0, Paper = 1, Scissors = 2
    int opponent = line[0] - 'A';

    // action: 0 = lose, 1 = draw, 2 = win
    int action = line[2] - 'X';

    // 0 = draw -> 3 points
    // 1 = win  -> 6 points
    // 2 = lose -> 0 points
    int me = (opponent + action + 2) % 3;
    int score = (me + 1) + (action * 3);

    total += score;

    line = &line[4];
  }

  return total;
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  const char *input = "A Y\n"
                      "B X\n"
                      "C Z\n";

  assert(p2(input) == 12);
}
#endif
