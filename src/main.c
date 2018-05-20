#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <switch.h>

extern bool rust_appletMainLoop();
extern void rust_hidScanInput();
extern void rust_gfxInitDefault();
extern void rust_gfxFlushBuffers();
extern void rust_gfxSwapBuffers();
extern void rust_gfxWaitForVsync();
extern void rust_gfxExit();
extern void like_a_main();

int main(int argc, char **argv) {
	rust_gfxInitDefault();
	consoleInit(NULL);
	printf("\x1b[16;16HPress PLUS to exit.");

	like_a_main();

	// // Main loop
	// while(rust_appletMainLoop()) {
	// 	//Scan all the inputs. This should be done once for each frame
	// 	rust_hidScanInput();

	// 	//hidKeysDown returns information about which buttons have been just pressed (and they weren't in the previous frame)
	// 	u32 kDown = hidKeysDown(CONTROLLER_P1_AUTO);

	// 	if (kDown & KEY_PLUS) break; // break in order to return to hbmenu

	// 	// do stuff
	// 	printf("hello\n");

	// 	rust_gfxFlushBuffers();
	// 	rust_gfxSwapBuffers();
	// 	rust_gfxWaitForVsync();
	// }

	rust_gfxExit();
	return 0;
}

