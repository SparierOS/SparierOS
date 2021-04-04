// SPDX-License-Identifier: MIT

.option norvc
.section .data

.section .text._boot
.global _start
_start:
    // We currently only work with one CPU so halt any additional CPUs
    csrr    t0, mhartid         // get current hart_id
    bnez    t0, 3f              // if hart_id != 0 then endless loop
    
    csrw	satp, zero          // disable address translation

.option push                    // initialize the gp register which is usually constant, with 
.option norelax                 // relaxion this might get replaced by `mov gp,gp` 
    la		gp, __global_pointer
.option pop

    // Setup the Stack
    la      sp, __stack_top
    
    // Use traps to jump into Rust
    // We set the `previous` fields because those will be set if we call `mret` a few lines down
    // 0b11 << 11   MPP (Previous Protection Mode) is 3 (M mode)
    li		t0, 0b11 << 11
    csrrs   x0, mstatus, t0
    // Programm counter is set to `runtime_init()` declared in `src/runtime.rs`
    la      t0, runtime_init
    csrw	mepc, t0
    // If we are ever returning go to endless loop
    la      ra, 3f 
    mret

3:  // endless loop
    wfi                         // wait for interrupt 
    j       3b                  // jump back to loop start
    
