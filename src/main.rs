// SPDX-License-Identifier: MIT

//! The main kernel binary

#![feature(asm)]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![no_main]
#![no_std]

// the entry point is in `mod cpu`

#[macro_use]
mod console;

mod bsp;
mod cpu;
mod driver;
mod memory;
mod panic;
mod runtime;
mod synchronization;
mod syscall;

/// This is where the register will be safed on traps in kernel mode
static mut KERNEL_REGISTERS: [usize; 31] =  [0; 31];

/// Kernel initialization code.
///
/// # Safety
///
/// - Only one hart must be active
unsafe fn kernel_init() -> ! {
    cpu::setup_trap_vector(KERNEL_REGISTERS.as_mut_ptr()); // Setup trap handlers before anything that could cause exceptions
    driver::init();

    kernel_main()
}

/// Main Function, executed after initializing basic runtime and drivers
fn kernel_main() -> ! {
    println!("[+] Kernel started");

    println!("[+] Driver initialization:");
    use driver::interface::DriverManager;
    for device in bsp::driver::driver_manager().list_drivers().iter() {
        if device.initialized() {
            println!("    [+] {} : successfull", device.name());
        } else {
            println!("    [+] {} : failed", device.name());
        }
    }

    // Safety: depends on the implementation of enter_into_usermode
    unsafe {
        DEMO_REGISTERS[1] = DEMO_STACK.as_mut_ptr().add(DEMO_STACK.len() - 1) as usize; // Setup stack
        DEMO_REGISTERS[2] = 0; // Global pointer will hopefully not be used
        let entry_pc = user_demo as fn() as usize;
        println!("[+] Entering usermode at {:#x}", entry_pc);
        cpu::enter_into_usermode(DEMO_REGISTERS.as_mut_ptr(), entry_pc);
    }
}

/// Handles traps. Gets called by the trap handler in 'traps.S'
#[no_mangle]
pub extern "C" fn kernel_trap_handler(registers: *mut usize, pc: usize, cause: u64, previous_mode: u64) -> ! {
    if cause == 8 && previous_mode == 0 { // User Syscall
        // Safety:
        // - registers is always a value previously already written to from index 0 to 31
        // - everything will be initialized because we come from usermode
        unsafe {
            syscall::handle_system_call(registers.add(9).read_volatile(), registers);
            cpu::return_from_trap(registers, pc + 4); // reenter usermode after the ecall instruction
        }
    }
    panic!("Unhandled trap {:#x} {:#x} {:#x}", pc, cause, previous_mode);
}

// This part would be part of process handling / program loading

static mut DEMO_STACK: [usize; 1024] = [0; 1024];
static mut DEMO_REGISTERS: [usize; 31] =  [0; 31];

fn user_demo() {
    // Safety: depends on the implementation of syscall and the trap and syscall handeling
    unsafe {
        cpu::syscall(0); // Print 'Hello world!'
        cpu::syscall(1, 2 + 3); // Print integer
        cpu::syscall(2); // Exit
    }
}

