#include "parser.h"
#include "base.h"

static bool is_digit(char c) {
  return c >= '0' && c <= '9';
}

static uint64_t parse_number(const char *line[]) {
  const char *ptr = *line;
  expect(is_digit(*ptr), "must be a number");
  uint64_t number = 0;

  while (is_digit(*ptr)) {
    number = 10 * number + *ptr - '0';
    ptr++;
  }

  *line = ptr;

  return number;
}

static bool parse_consume(const char **ptr, char expected) {
  if (**ptr != expected) return false;
  (*ptr)++;
  return true;
}

ParseResult parse_ranges(const char *input) {
  ParseResult result = (ParseResult){.rest = input};
  const char *ptr = input;

  if (!*ptr || !is_digit(*ptr)) return (ParseResult){0};

  while (*ptr && *ptr != '\n') {
    result.first.start = parse_number(&ptr);
    if (!parse_consume(&ptr, '-')) return result;
    result.first.end = parse_number(&ptr);

    if (!parse_consume(&ptr, ',')) return result;

    result.second.start = parse_number(&ptr);
    if (!parse_consume(&ptr, '-')) return result;
    result.second.end = parse_number(&ptr);
  }

  parse_consume(&ptr, '\n');

  result.ok = true;
  result.rest = ptr;

  return result;
}

#ifdef TEST
#include <assert.h>

void parser_tests(void) {
  ParseResult result;
  const char *input = "2-4,6-8\n"
                      "2-3,4-5\n";

  result = parse_ranges(input);
  assert(result.first.start == 2);
  assert(result.first.end == 4);
  assert(result.second.start == 6);
  assert(result.second.end == 8);
  assert(result.ok);

  result = parse_ranges(result.rest);
  assert(result.first.start == 2);
  assert(result.first.end == 3);
  assert(result.second.start == 4);
  assert(result.second.end == 5);
  assert(result.ok);

  result = parse_ranges(result.rest);
  assert(result.ok == false);
}
#endif
