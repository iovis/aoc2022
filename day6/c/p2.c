#include "p2.h"

#include "base.h"
#include "set.h"

#include <stdio.h>

size_t p2(const char *input) {
  size_t window_size = 14;

  for (size_t i = 0;; i++) {
    expect(input[i + window_size - 1] != '\0', "Reached end of input");

    uint32_t set = 0;
    set_init(&set, &input[i], window_size);

    if (set_count(set) == window_size) return i + window_size;
  }
}

#ifdef TEST
#include <assert.h>

void p2_tests(void) {
  assert(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19);
  assert(p2("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23);
  assert(p2("nppdvjthqldpwncqszvftbrmjlhg") == 23);
  assert(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29);
  assert(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26);
}
#endif
