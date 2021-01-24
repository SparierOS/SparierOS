// SPDX-License-Identifier: MIT

.option norvc
.section .data

.section .text._boot
.global _start
_start:
1:  wfi             // wait for an interrupt 
    j   1b          // jump back to 1
    