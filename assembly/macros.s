
.macro push register
    sw \register, 0(sp)
    addi sp, sp, -16
.endm

.macro pop register
    addi sp, sp, 16
    lw \register, 0(sp)
.endm
