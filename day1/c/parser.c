#include "parser.h"
#include <ctype.h>
#include <stdint.h>

static uint64_t parse_number(const char *line[]) {
  uint64_t number = 0;
  const char *ptr = *line;

  while (isdigit(*ptr)) {
    number = 10 * number + (*ptr - '0');
    ptr++;
  }

  if (*ptr == '\n') ptr++;

  *line = ptr;

  return number;
}

ResultParseGroup parse_group(const char *input) {
  const char *ptr = input;

  if (!isdigit(*ptr)) {
    return (ResultParseGroup){
        .ok = false,
        .rest = input,
    };
  }

  uint64_t sum = 0;
  while (isdigit(*ptr)) {
    sum += parse_number(&ptr);
  }

  if (*ptr == '\n') ptr++;

  return (ResultParseGroup){
      .ok = true,
      .group = sum,
      .rest = ptr,
  };
}

#ifdef TEST
#include <assert.h>

static void parse_number_test(void) {
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

  assert(parse_number(&input) == 1000);
  assert(parse_number(&input) == 2000);
  assert(parse_number(&input) == 3000);
}

static void parse_group_test(void) {
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

  ResultParseGroup result;
  result = parse_group(input);
  assert(result.ok);
  assert(result.group == 6000);

  result = parse_group(result.rest);
  assert(result.ok);
  assert(result.group == 4000);

  result = parse_group(result.rest);
  assert(result.ok);
  assert(result.group == 11000);

  result = parse_group(result.rest);
  assert(result.ok);
  assert(result.group == 24000);

  result = parse_group(result.rest);
  assert(result.ok);
  assert(result.group == 10000);

  result = parse_group(result.rest);
  assert(!result.ok);
}

void parser_tests(void) {
  parse_number_test();
  parse_group_test();
}
#endif
