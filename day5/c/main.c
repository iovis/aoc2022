#include "lib.h"

#include "p1.h"
#include "p2.h"
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

  String p2_result = p2(input);
  defer string_free(&p2_result);

  printf("p2 = %s\n", p2_result.cstr);

  return 0;
}
