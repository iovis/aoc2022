#pragma once

#include <stdint.h>

typedef struct {
  bool ok;
  uint64_t group;
  const char *rest;
} ResultParseGroup;

ResultParseGroup parse_group(const char *line);

#ifdef TEST
void parser_tests(void);
#endif
