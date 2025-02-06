# Simple Risc-V I32 Emulator!
## and other unfinished stuff

The "risc-v" folder contains the RV I32 machine emulator. It's written in a way to allow easy implementation of other Risc-V extentions and to be eazy to undersand. I tried to comment as much as I could (and was disciplined enought that day to do so) and I think I did a pretty good job at it! =^^= 

Also, the machine has a memory bus, to which you can map different "devices". For now, I implemented 64KiB ROM chip and a 64KiB RAM chip. I started to implement a SDL powered screen on which you could set colored pixels, just for fun =^^= ... buuut I never finished it ( ´ ω ` )

The "assembly" folder contains some RV assembly files that I wrote to test the processor. You can check Makefile there to see how to build them.

The "machine-viewer" is a program that I used to debug the processor. I tried to rewrite it and I never finished it, so the tool isn't working for now. This is a TUI tool, because I didn't want to waste time writing an SDL app.

The "virtual-console" was supposed to be a game console powered by RV emulator, something like PICO-8 or TIC-80. I never finished it.

If you use my code in your project or you want to develop the emulator futher, please reference me as an original author. Many thanks =^^=