#pragma once

#include "str.h"

typedef struct {
  // [owned]: string_array_free(result.containers)
  String *containers;
  const char *rest;
  bool ok;
} ParseContainersResult;

typedef struct {
  size_t amount;
  size_t from;
  size_t to;
} Operation;

typedef struct {
  // [owned]: arrfree(result.operations)
  Operation *operations;
  const char *rest;
  bool ok;
} ParseOperationsResult;

ParseContainersResult parse_containers(const char *input);
ParseOperationsResult parse_operations(const char *input);

#ifdef TEST
void parser_tests(void);
#endif
