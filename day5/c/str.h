#pragma once

#include <stddef.h>

typedef struct {
  char *cstr;
  size_t len;
  size_t capacity;
} String;

typedef struct {
  const char *ptr;
  size_t len;
} str_t;

typedef String *StringArray;

String string_new(size_t capacity);
String string_from_cstr(const char *cstr);
String string_from_str(const str_t *str);
str_t string_as_ref(const String *self);

str_t str_new(const char *ptr, size_t len);
str_t str_from_cstr(const char *cstr);
str_t str_delim(const char *cstr, char delim);

// Precondition: cstr must not point into self->cstr
void string_append_cstr(String *self, const char *cstr);
void string_append_char(String *self, char c);
// Precondition: str->ptr must not point into self->cstr
void string_append_str(String *self, const str_t *str);
char string_pop(String *self);
void string_chop(String *self, size_t n);

void string_clear(String *self);
void string_free(String *self);
void string_array_free(String *self);

#ifdef TEST
void string_tests(void);
#endif
