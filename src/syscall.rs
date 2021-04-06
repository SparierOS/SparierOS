// SPDX-License-Identifier: MIT

//! The system call handler

/// Handles all user syscalls
pub fn handle_system_call(kind: usize, registers: *const usize) {
    // These syscalls are obviously just temporary
    // Maybe a table lookup would be preferred here (but not as safe)
    match kind {
        0 => { // Print 'Hello world!'
            println!("Hello world!");
        }
        1 => { // Print integer
            let number;
            // Safety: registers is always a value previously already written to from index 0 to 31
            unsafe { number = registers.add(10).read_volatile(); }
            println!("{}", number);
        }
        2 => { // Exit
            println!("[+] User code exited");
            // Just loop infinitely
            unsafe { asm!("1: wfi \n j 1b") };
            unreachable!()
        }
        _ => {
            panic!("Unknown syscall {:#x}", kind)
        }
    }
}

