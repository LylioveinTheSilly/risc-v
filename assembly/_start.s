.globl _start
_start: 
    li sp, 0x8FFC   # setup stack pointer
    j main
    