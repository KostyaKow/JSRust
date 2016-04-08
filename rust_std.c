#include <stdio.h>
#include <stdint.h>

uint32_t c_add_ints(uint32_t a, uint32_t b) { return a + b; }

void c_putchar(char c) { putchar(c); }

char c_int_to_char(int a) { return (char)(a % 255); }

char c_int_lt(int a, int b) { return a < b; }
char c_int_gt(int a, int b) { return a > b; }
char c_int_eq(int a, int b) { return a == b; }

void c_print_ln(char** s) { printf("%s\n", *s); }

void c_panic(char *s) {
   printf("panic: %s");
}

//printf("Hello world! %c", c);

/*int main() {
   return hello();
}*/
