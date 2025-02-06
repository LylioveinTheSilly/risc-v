run:
	mkdir -p build

	riscv64-linux-gnu-as -march=rv32i _start.s -o build/_start.o 
	riscv64-linux-gnu-as -march=rv32i lib.s    -o build/lib.o 
	riscv64-linux-gnu-as -march=rv32i main.s   -o build/main.o 

	riscv64-linux-gnu-ld -m elf32lriscv -T virtual-console.cfg -o build/rom.elf \
		build/_start.o \
		build/lib.o \
		build/main.o

	riscv64-linux-gnu-objcopy -O binary build/rom.elf build/rom.bin

clean:
	rm build/*
