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

    // Setup machine trap vector
    la      t0, machine_trap_handler
    csrw    mtvec, t0
    
    // Use traps to jump into Rust
    // We set the `previous` fields because those will be set if we call `mret` a few lines down
    // 0b01 << 11   MPP (Previous Protection Mode) is 1 (S mode)
    li		t0, 0b11 << 11
    csrrc   x0, mstatus, t0
    li		t0, 0b01 << 11
    csrrs   x0, mstatus, t0 // Clear and set to avoid changing other fields
    // Setup trap delegation
    li		t0, 0xffff
    csrw    medeleg, t0 // Delegate all exceptions for now
    csrw    mideleg, t0 // Delegate all interrupts for now
    // Programm counter is set to `runtime_init()` declared in `src/runtime.rs`
    la      t0, runtime_init
    csrw	mepc, t0
    // If we are ever returning go to endless loop
    la      ra, 3f 
    mret

3:  // endless loop
    wfi                         // wait for interrupt 
    j       3b                  // jump back to loop start

.align 4 // Alignment requirements are implementation defined (4 is the minimum)
machine_trap_handler:
    j 3b

