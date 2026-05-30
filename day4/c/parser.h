#pragma once

#include "range.h"

typedef struct {
  Range first;
  Range second;
  bool ok;
  const char *rest;
} ParseResult;

ParseResult parse_ranges(const char *input);

#ifdef TEST
void parser_tests(void);
#endif
