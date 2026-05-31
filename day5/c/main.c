#include "lib.h"

#include "p1.h"
#include "str.h"

#include <stddefer.h>
#include <stdio.h>

static const char input[] = {
#embed "../input.txt" suffix(, )
    '\0'
};

int main(void) {
  String p1_result = p1(input);
  defer string_free(&p1_result);

  printf("p1 = %s\n", p1_result.cstr);
  // printf("p2 = %lu\n", p2(input).str);

  return 0;
}
