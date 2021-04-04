// SPDX-License-Identifier: MIT

.option norvc
.section .bss
.align 8
kernel_register_save:
.zero 256 // Size will depend on the number of harts

.section .text
.global setup_trap_vector
.global return_from_trap
.global enter_into_usermode
.global syscall

// Setup the trap vector and save the global and stack pointer
// Should be setup once per hart
setup_trap_vector:
    // Store some important kernel registers (Once per hart)
    la t0, kernel_register_save
    csrr t1, mhartid
    slli t1, t1, 4 // 2^4 = 16 = 8*2
    add t0, t0, t1
    sd sp,  0(t0)
    sd gp,  8(t0)
    // Setup trap vector to point to trap_handler
    la t0, trap_handler
    csrw mtvec, t0
    // Setup location for storing kernel register state
    csrw mscratch, a0
    ret

// Simply execute ecall
syscall:
    ecall
    ret

// Enter usermode at a certain address
enter_into_usermode:
    // 0b00 << 11   MPP (Previous Protection Mode) is 0 (U mode)
    li t0, 0b11 << 11
    csrrc x0, mstatus, t0
    // Go on into return_from_trap

// Return from a trap to a certain address
return_from_trap:
    csrw mepc, a1
    csrw mscratch, a0
    // Restore user registers
    ld  x1,   0(a0)
    ld  x2,   8(a0)
    ld  x3,  16(a0)
    ld  x4,  24(a0)
    ld  x5,  32(a0)
    ld  x6,  40(a0)
    ld  x7,  48(a0)
    ld  x8,  56(a0)
    ld  x9,  64(a0)
    ld x11,  80(a0)
    ld x12,  88(a0)
    ld x13,  96(a0)
    ld x14, 104(a0)
    ld x15, 112(a0)
    ld x16, 120(a0)
    ld x17, 128(a0)
    ld x18, 136(a0)
    ld x19, 144(a0)
    ld x20, 152(a0)
    ld x21, 160(a0)
    ld x22, 168(a0)
    ld x23, 176(a0)
    ld x24, 184(a0)
    ld x25, 192(a0)
    ld x26, 200(a0)
    ld x27, 208(a0)
    ld x28, 216(a0)
    ld x29, 224(a0)
    ld x30, 232(a0)
    ld x31, 240(a0)
    ld x10,  72(a0) // x10 == a0 and must therefore be loaded last
    mret

.align 4 // Alignment requirements are implementation defined (4 is the minimum)
// Handle a trap by saving user registers and calling the kernel trap handler
trap_handler:
    csrrw x1, mscratch, x1 // Swap mscratch with x1
    // Save user registers
    sd  x2,   8(x1)
    sd  x3,  16(x1)
    sd  x4,  24(x1)
    sd  x5,  32(x1)
    sd  x6,  40(x1)
    sd  x7,  48(x1)
    sd  x8,  56(x1)
    sd  x9,  64(x1)
    sd x10,  72(x1)
    sd x11,  80(x1)
    sd x12,  88(x1)
    sd x13,  96(x1)
    sd x14, 104(x1)
    sd x15, 112(x1)
    sd x16, 120(x1)
    sd x17, 128(x1)
    sd x18, 136(x1)
    sd x19, 144(x1)
    sd x20, 152(x1)
    sd x21, 160(x1)
    sd x22, 168(x1)
    sd x23, 176(x1)
    sd x24, 184(x1)
    sd x25, 192(x1)
    sd x26, 200(x1)
    sd x27, 208(x1)
    sd x28, 216(x1)
    sd x29, 224(x1)
    sd x30, 232(x1)
    sd x31, 240(x1)
    mv a0, x1
    csrrw x1, mscratch, x1 // Load the user register value for x1
    sd x1, 0(a0) // Save the x1 user value
    // Restore some important kernel registers (Different per hart)
    la t0, kernel_register_save
    csrr t1, mhartid
    slli t1, t1, 4 // 2^4 = 16 = 8*2
    add t0, t0, t1
    ld sp,  0(t0)
    ld gp,  8(t0)
    // Set arguments pc, cause and previous_mode
    csrr a1, mepc
    csrr a2, mcause
    csrr a3, mstatus
    slli a3, a3, 11
    andi a3, a3, 0b11
    j kernel_trap_handler // Call kernel handler

