#include "p2.h"

#include "base.h"
#include "parser.h"
#include "stb_ds.h"
#include "str.h"

#include <stddefer.h>

String p2(const char *input) {
  ParseContainersResult containers_result = parse_containers(input);
  expect(containers_result.ok);
  String *containers = containers_result.containers;
  defer string_array_free(containers);

  const char *line = containers_result.rest;
  while (true) {
    ParseOperationResult operation_result = parse_operation(line);
    if (!operation_result.ok) break;

    Operation operation = operation_result.operation;
    String *from = &containers[operation.from];
    String *to = &containers[operation.to];

    str_t to_move = (str_t){
        .ptr = &from->cstr[from->len - operation.amount],
        .len = operation.amount,
    };
    string_append_str(to, &to_move);
    string_chop(from, operation.amount);

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

void p2_tests(void) {
  const char *input = "    [D]\n"
                      "[N] [C]\n"
                      "[Z] [M] [P]\n"
                      " 1   2   3\n"
                      "\n"
                      "move 1 from 2 to 1\n"
                      "move 3 from 1 to 3\n"
                      "move 2 from 2 to 1\n"
                      "move 1 from 1 to 2\n";

  String result = p2(input);
  defer string_free(&result);

  assert(strcmp(result.cstr, "MCD") == 0);
}
#endif
