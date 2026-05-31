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

  ParseOperationsResult operations_result = parse_operations(containers_result.rest);
  expect(operations_result.ok);
  Operation *operations = operations_result.operations;
  defer arrfree(operations);

  for (ptrdiff_t i = 0; i < arrlen(operations); i++) {
    for (size_t j = 0; j < operations[i].amount; j++) {
      char container = string_pop(&containers[operations[i].from]);
      string_append_char(&containers[operations[i].to], container);
    }
  }

  String result = string_new(arrlen(containers));
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
