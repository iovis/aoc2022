#include "lib.h"

#include "p1.h"
#include "p2.h"
#include "range.h"

#include <stdio.h>

int main(void) {
  range_tests();
  parser_tests();
  p1_tests();
  p2_tests();

  puts("ok");
  return 0;
}
