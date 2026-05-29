#include "bitmap.h"

#include "base.h"

#include <ctype.h>

// Use bitmap to set letters found
// a-z -> 1-26
// A-Z -> 27-52
uint64_t parse_line(size_t n, const char line[static n]) {
  uint64_t bitmap = 0;

  for (size_t i = 0; i < n; i++) {
    if (islower(line[i])) {
      bitmap |= 1ULL << (line[i] - 'a');
    } else if (isupper(line[i])) {
      bitmap |= 1ULL << (line[i] - 'A' + 26);
    } else {
      expect(false, "Expected alpha, got %c:\n%.*s", line[i], (int)n, line);
    }
  }

  return bitmap;
}

void bitmap_print(uint64_t n) {
  printf("0b");

  for (int i = 63; i >= 0; i--) {
    putchar((n & (1ULL << i)) ? '1' : '0');
    if (i % 8 == 0 && i != 0) putchar('\'');
  }

  putchar('\n');
}

#ifdef TEST
#include <assert.h>
#include <string.h>

void bitmap_tests(void) {
  const char *input;

  input = "vJrwpWtwJgWrhcsFMMfFFhFp";
  assert(parse_line(strlen(input), input) == 0b00000000'00000001'00000000'01001000'10000000'01101110'10000000'11100100);
}
#endif
