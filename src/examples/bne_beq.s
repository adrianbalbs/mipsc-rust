main:
        addi $t0, $zero, 5
        bne $t1, $t0, 2
        bne $t2, $t0, 5
        beq $t3, $t0, 5
        beq $t4, $t0, 2
