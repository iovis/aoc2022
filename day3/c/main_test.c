#include "lib.h"

#include "bitmap.h"
#include "p1.h"
#include "p2.h"

#include <stdio.h>

int main(void) {
  bitmap_tests();
  p1_tests();
  p2_tests();

  puts("ok");
  return 0;
}
