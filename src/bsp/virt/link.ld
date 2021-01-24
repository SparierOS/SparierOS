/* SPDX-License-Identifier: MIT */

OUTPUT_ARCH( "riscv" )

/* define entry point */
ENTRY( _start )

MEMORY {
    ram  (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

/* List of Program Headers 
 *  - PTLOAD: `Indicates that this program header describes a segment to be loaded from the file.`
 */
PHDRS {
    text PT_LOAD;
    data PT_LOAD;
    bss PT_LOAD;
}

SECTIONS {

    /* text section with .text._boot at the very start to ensure its at address 0x80000000 */
    .text : {
        *(.text._boot) *(.text*)
    } >ram AT>ram :text

    /* I don't know what this is for but we have to move it away from the top */
    .eh_frame : {
        *(.rodata .rodata.*)
    } >ram AT>ram :text

    PROVIDE(__global_pointer = .);

    .rodata : {
        *(.rodata .rodata.*)
    } >ram AT>ram :text

    .data : {
	    . = ALIGN(4096);
        *(.sdata .sdata.*) *(.data .data.*)
    } >ram AT>ram :data

    .bss ALIGN(8): {
        PROVIDE(__bss_start = .);
        *(.sbss .sbss.*) *(.bss .bss.*)

        /* Make sure BSS is never empty */
        . += 8;
        . = ALIGN(8);
        PROVIDE(__bss_end = .);
    } >ram AT>ram :bss
    
    /* NOTE: stacks grows from higher to lower memory, for now fixed 524 KiB should do */
    PROVIDE(__stack_top    = ORIGIN(ram) + LENGTH(ram));
    PROVIDE(__stack_bottom = __stack_top - 0x80000);

    PROVIDE(__heap_start = __bss_end);
    PROVIDE(__heap_end   = __stack_bottom);
}