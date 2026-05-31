#include "parser.h"

#include "base.h"
#include "stb_ds.h"
#include "str.h"
#include <stddef.h>
#include <stddefer.h>
#include <string.h>

static bool is_digit(char c) {
  return c >= '0' && c <= '9';
}

static size_t parse_number(const char **ptr) {
  size_t number = 0;

  while (is_digit(**ptr)) {
    number = 10 * number + **ptr - '0';
    (*ptr)++;
  }

  return number;
}

static bool is_container(char c) {
  return c >= 'A' && c <= 'Z';
}

static bool parse_consume(const char **ptr, const char *tag) {
  while (**ptr && *tag) {
    if (**ptr != *tag) return false;
    (*ptr)++;
    tag++;
  }

  // tag was consumed
  return *tag == '\0';
}

ParseContainersResult parse_containers(const char *input) {
  ParseContainersResult result = {.rest = input};
  const char *ptr = input;
  str_t *lines = nullptr;
  defer arrfree(lines);

  // store container lines
  while (*ptr) {
    if (*ptr == '\n') {
      result.rest = ptr + 1;
      break;
    }

    str_t line = str_delim(ptr, '\n');
    arrpush(lines, line);

    ptr += line.len + 1;
  }

  str_t indexes = arrpop(lines);
  size_t container_idx = 0;

  for (size_t j = 0; j < indexes.len; j++) {
    if (!is_digit(indexes.ptr[j])) continue;

    arrput(result.containers, string_new(arrlen(lines)));
    for (ptrdiff_t i = arrlen(lines) - 1; i >= 0; i--) {
      if (lines[i].len <= j) break;
      if (!is_container(lines[i].ptr[j])) break;

      string_append_char(&result.containers[container_idx], lines[i].ptr[j]);
    }

    container_idx++;
  }

  result.ok = true;

  return result;
}

ParseOperationsResult parse_operations(const char *input) {
  ParseOperationsResult result = {
      .operations = nullptr,
      .rest = input,
  };

  const char *ptr = input;

  while (*ptr) {
    Operation operation = {0};

    if (!parse_consume(&ptr, "move ")) return result;
    operation.amount = parse_number(&ptr);
    if (!operation.amount) return result;

    if (!parse_consume(&ptr, " from ")) return result;
    operation.from = parse_number(&ptr);
    if (!operation.from) return result;
    operation.from -= 1;

    if (!parse_consume(&ptr, " to ")) return result;
    operation.to = parse_number(&ptr);
    if (!operation.to) return result;
    operation.to--;

    if (!parse_consume(&ptr, "\n")) return result;

    arrput(result.operations, operation);
  }

  result.ok = true;
  result.rest = ptr;

  return result;
}

#ifdef TEST
#include <assert.h>

static void parse_containers_test(void) {
  const char *input = "    [D]\n"
                      "[N] [C]\n"
                      "[Z] [M] [P]\n"
                      " 1   2   3\n"
                      "\n"
                      "move 1 from 2 to 1\n"
                      "move 3 from 1 to 3\n"
                      "move 2 from 2 to 1\n"
                      "move 1 from 1 to 2\n";

  ParseContainersResult result = parse_containers(input);
  String *containers = result.containers;
  defer string_array_free(containers);

  assert(result.ok);
  assert(arrlen(containers) == 3);
  assert(strcmp(containers[0].cstr, "ZN") == 0);
  assert(strcmp(containers[1].cstr, "MCD") == 0);
  assert(strcmp(containers[2].cstr, "P") == 0);
  assert(
      strcmp(
          result.rest,
          "move 1 from 2 to 1\n"
          "move 3 from 1 to 3\n"
          "move 2 from 2 to 1\n"
          "move 1 from 1 to 2\n"
      )
      == 0
  );
}

void parse_operations_test(void) {
  const char *input = "move 1 from 2 to 1\n"
                      "move 3 from 1 to 3\n"
                      "move 2 from 2 to 1\n"
                      "move 1 from 1 to 2\n";

  ParseOperationsResult result = parse_operations(input);
  Operation *operations = result.operations;
  defer arrfree(operations);

  assert(result.ok);
  assert(arrlen(operations) == 4);

  assert(operations[0].amount == 1);
  assert(operations[0].from == 1);
  assert(operations[0].to == 0);

  assert(operations[1].amount == 3);
  assert(operations[1].from == 0);
  assert(operations[1].to == 2);

  assert(operations[2].amount == 2);
  assert(operations[2].from == 1);
  assert(operations[2].to == 0);

  assert(operations[3].amount == 1);
  assert(operations[3].from == 0);
  assert(operations[3].to == 1);
}

void parser_tests(void) {
  parse_containers_test();
  parse_operations_test();
}
#endif
