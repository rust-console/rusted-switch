#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <switch.h>

extern void rust_main();

int main(int argc, char **argv) {
	// I'm not sure why but if I don't call consoleInit() from C
	// the <switch.h> library is not linked properly ¯\_(ツ)_/¯
	gfxInitDefault();
	consoleInit(NULL);

	rust_main();

	gfxExit();
	return 0;
}

