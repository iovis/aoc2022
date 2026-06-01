#include "p1.h"

#include "base.h"
#include "parser.h"
#include "stb_ds.h"
#include "str.h"
#include <stddef.h>
#include <stddefer.h>

String p1(const char *input) {
  ParseContainersResult containers_result = parse_containers(input);
  expect(containers_result.ok);
  String *containers = containers_result.containers;
  defer string_array_free(containers);

  const char *line = containers_result.rest;
  while (true) {
    ParseOperationResult operation_result = parse_operation(line);
    if (!operation_result.ok) break;

    Operation operation = operation_result.operation;

    for (size_t j = 0; j < operation.amount; j++) {
      char container = string_pop(&containers[operation.from]);
      string_append_char(&containers[operation.to], container);
    }

    line = operation_result.rest;
  }

  String result = string_new(64);
  for (ptrdiff_t i = 0; i < arrlen(containers); i++) {
    char container = string_pop(&containers[i]);
    string_append_char(&result, container);
  }

  return result;
}

#ifdef TEST
#include <assert.h>
#include <string.h>

void p1_tests(void) {
  const char *input = "    [D]\n"
                      "[N] [C]\n"
                      "[Z] [M] [P]\n"
                      " 1   2   3\n"
                      "\n"
                      "move 1 from 2 to 1\n"
                      "move 3 from 1 to 3\n"
                      "move 2 from 2 to 1\n"
                      "move 1 from 1 to 2\n";

  String result = p1(input);
  defer string_free(&result);

  assert(strcmp(result.cstr, "CMZ") == 0);
}
#endif
