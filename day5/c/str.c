#include "str.h"

#include "base.h"
#include "stb_ds.h"

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

String string_new(size_t capacity) {
  String string = (String){.capacity = capacity};
  expect(string.capacity < SIZE_MAX - 1, "string too large");

  string.cstr = malloc(string.capacity + 1);
  expect(string.cstr != nullptr, "failed to allocate String");

  string.cstr[0] = '\0';

  return string;
}

String string_from_cstr(const char *cstr) {
  String string = (String){0};

  string.len = strlen(cstr);
  string.capacity = string.len;
  expect(string.capacity < SIZE_MAX - 1, "string too large");

  string.cstr = malloc(string.capacity + 1);
  expect(string.cstr != nullptr, "failed to allocate String");

  memcpy(string.cstr, cstr, string.len);
  string.cstr[string.len] = '\0';

  return string;
}

String string_from_str(const str_t *str) {
  String string = (String){
      .len = str->len,
      .capacity = str->len,
  };

  string.cstr = malloc(string.capacity + 1);
  expect(string.cstr != nullptr, "failed to allocate String");

  memcpy(string.cstr, str->ptr, string.len);
  string.cstr[string.len] = '\0';

  return string;
}

str_t string_as_ref(const String *self) {
  return (str_t){
      .ptr = self->cstr,
      .len = self->len,
  };
}

str_t str_new(const char *ptr, size_t len) {
  return (str_t){
      .ptr = ptr,
      .len = len,
  };
}

str_t str_from_cstr(const char *cstr) {
  return (str_t){
      .ptr = cstr,
      .len = strlen(cstr),
  };
}

str_t str_delim(const char *cstr, char delim) {
  const char *end = strchrnul(cstr, delim);
  return (str_t){
      .ptr = cstr,
      .len = end - cstr,
  };
}

static void string_grow(String *self) {
  size_t new_capacity = (self->capacity == 0) ? 16 : 2 * self->capacity;
  expect(new_capacity > self->capacity, "string too large (overflow)");

  char *new_cstr = realloc(self->cstr, new_capacity + 1);
  expect(new_cstr != nullptr, "failed to grow string");

  self->cstr = new_cstr;
  self->capacity = new_capacity;
}

void string_append_char(String *self, char c) {
  expect(1 <= SIZE_MAX - 1 - self->len, "string too large");

  size_t needed = self->len + 1;
  while (needed > self->capacity) string_grow(self);

  self->cstr[self->len] = c;
  self->len++;
  self->cstr[self->len] = '\0';
}

void string_append_cstr(String *self, const char *cstr) {
  size_t cstr_len = strlen(cstr);
  if (cstr_len == 0) return;
  expect(cstr_len <= SIZE_MAX - 1 - self->len, "string too large");

  size_t needed = self->len + cstr_len;
  while (needed > self->capacity) string_grow(self);

  memcpy(&self->cstr[self->len], cstr, cstr_len);
  self->len += cstr_len;
  self->cstr[self->len] = '\0';
}

void string_append_str(String *self, const str_t *str) {
  if (str->len == 0) return;
  expect(str->len <= SIZE_MAX - 1 - self->len, "string too large");

  size_t needed = self->len + str->len;
  while (needed > self->capacity) string_grow(self);

  memcpy(&self->cstr[self->len], str->ptr, str->len);
  self->len += str->len;
  self->cstr[self->len] = '\0';
}

char string_pop(String *self) {
  if (self->len == 0) return '\0';

  char c = self->cstr[self->len - 1];
  self->len--;
  self->cstr[self->len] = '\0';

  return c;
}

void string_chop(String *self, size_t n) {
  if (self->capacity == 0) return;

  if (n >= self->len) {
    self->len = 0;
  } else {
    self->len -= n;
  }

  self->cstr[self->len] = '\0';
}

void string_clear(String *self) {
  if (self->capacity == 0) return;

  self->cstr[0] = '\0';
  self->len = 0;
}

void string_free(String *self) {
  free(self->cstr);
  *self = (String){0};
}

void string_array_free(String *self) {
  for (int i = 0; i < arrlen(self); i++) {
    string_free(&self[i]);
  }

  arrfree(self);
}

#ifdef TEST
#include <assert.h>
#include <stddefer.h>

static void str_test(void) {
  const char *cstr = "Hello, World!";
  str_t str = str_from_cstr(cstr);
  assert(str.len == 13);
  assert(str.ptr == cstr);

  String string = string_from_str(&str);
  defer string_free(&string);
  assert(string.len == 13);
  assert(strcmp(string.cstr, cstr) == 0);

  str = string_as_ref(&string);
  assert(str.len == 13);
  assert(*str.ptr == *cstr);

  const char *lines = "Hello\nWorld";
  str = str_delim(lines, '\n');
  assert(str.len == 5);
  assert(*str.ptr == 'H');

  str = str_delim(&lines[str.len + 1], '\n');
  assert(str.len == 5);
  assert(*str.ptr == 'W');
}

static void string_test(void) {
  const char *cstr = "Hello!";
  String string = string_from_cstr(cstr);
  defer string_free(&string);

  assert(string.len == 6);
  assert(string.capacity == 6);
  assert(strcmp(string.cstr, "Hello!") == 0);

  char c = string_pop(&string);
  assert(string.len == 5);
  assert(c == '!');

  string_append_char(&string, ',');
  string_append_char(&string, ' ');
  assert(string.len == 7);
  assert(string.capacity == 12);
  assert(strcmp(string.cstr, "Hello, ") == 0);

  string_append_cstr(&string, "World");
  assert(string.len == 12);
  assert(string.capacity == 12);
  assert(strcmp(string.cstr, "Hello, World") == 0);

  string_clear(&string);
  assert(string.len == 0);
  assert(string.capacity == 12);
  assert(string.cstr[0] == '\0');

  c = string_pop(&string);
  assert(c == '\0');

  string_append_cstr(&string, "this is a test\n");
  assert(string.len == 15);
  assert(string.capacity == 24);
  assert(strcmp(string.cstr, "this is a test\n") == 0);

  str_t str = str_from_cstr(cstr);
  string_append_str(&string, &str);
  assert(string.len == 21);
  assert(string.capacity == 24);
  assert(strcmp(string.cstr, "this is a test\nHello!") == 0);

  string_chop(&string, 6);
  assert(string.len == 15);
  assert(string.capacity == 24);
  assert(strcmp(string.cstr, "this is a test\n") == 0);
}

void string_tests(void) {
  str_test();
  string_test();
}
#endif
