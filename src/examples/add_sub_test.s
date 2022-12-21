main:
  addi $t0, $zero, 4
  addi $t1, $zero, 5
  add  $t2, $t0, $t1
  sub  $t3, $t1, $t0
  slt  $t4, $t3, $t2
  slt  $t5, $t2, $t3
