main:
        addi $t0, $zero, 4
        addi $t1, $zero, 2
        div  $t0, $t1

        addi $t2, $zero, 3
        div  $t0, $t2

        mfhi $t3
        mflo $t4
