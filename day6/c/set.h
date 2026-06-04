#pragma once

#include <stdbit.h>
#include <stdint.h>

void set_init(uint32_t *self, const char *start, size_t n);
size_t set_count(uint32_t self);
void set_print(uint32_t n);
