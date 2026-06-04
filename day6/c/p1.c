#include "p1.h"

#include "base.h"
#include "set.h"

#include <stdio.h>

size_t p1(const char *input) {
  size_t window_size = 4;

  for (size_t i = 0;; i++) {
    expect(input[i + window_size - 1] != '\0', "Reached end of input");

    uint32_t set = 0;
    set_init(&set, &input[i], window_size);

    if (set_count(set) == window_size) return i + window_size;
  }
}

#ifdef TEST
#include <assert.h>

void p1_tests(void) {
  assert(p1("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5);
  assert(p1("nppdvjthqldpwncqszvftbrmjlhg") == 6);
  assert(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10);
  assert(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11);
}
#endif
