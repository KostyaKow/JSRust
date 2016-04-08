#include <stdio.h>

int c_add_ints(int a, int b) { return a + b; }

void c_putchar(char c) { putchar(c); }

char c_int_to_char(int a) { return (char)a; }

char c_int_lt(int a, int b) { return a < b; }
char c_int_gt(int a, int b) { return a > b; }
char c_int_eq(int a, int b) { return a == b; }

void c_print_ln(char** s) { printf("%s\n", *s); }

//printf("Hello world! %c", c);

/*int main() {
   return hello();
}*/
