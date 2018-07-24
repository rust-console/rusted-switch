#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <switch.h>
#include <time.h>

extern void rust_main();

int main(int argc, char **argv) {

	rust_main();

	return 0;
}

