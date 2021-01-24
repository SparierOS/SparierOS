# SPDX-License-Identifier: MIT

# Default Board is virt
BOARD ?= virt

# Settings depending on BOARD
ifeq ($(BOARD),virt)
	TARGET			= riscv64gc-unknown-none-elf
	ARCH			= riscv64gc
	LINKER_SCRIPT	= src/bsp/virt/link.ld
	QEMU_BIN		= qemu-system-riscv64
	QEMU_MACHINE	= virt
	QEMU_ARGS		= -cpu rv64 -smp 4 -m 128M -display none -bios none
	QEMU_DEVICES	= -serial stdio
endif

# build.rs depends on this
export LINKER_SCRIPT

# default flags passed to rustc
RUSTFLAGS		= -C link-arg=-T$(LINKER_SCRIPT) -D warnings -D missing_docs
QUICKRUSTFLAGS	= -C link-arg=-T$(LINKER_SCRIPT)

COMPILER_ARGS 	= --target=$(TARGET) 		\
	--features bsp_$(BOARD),arch_$(ARCH)	\
	--release

# the commands that can be run
RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)

# directory for the generated elf
KERNEL_ELF = target/$(TARGET)/release/sparier_os

.PHONY: all $(KERNEL_ELF) doc qemu clippy readelf check

all: $(KERNEL_ELF)

$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

doc:
	$(DOC_CMD) --document-private-items --target-dir target/docs

qemu: $(KERNEL_ELF)
	@$(QEMU_BIN) -M $(QEMU_MACHINE) $(QEMU_ARGS) $(QEMU_DEVICES) -kernel $(KERNEL_ELF)

debug:
	RUSTFLAGS="$(QUICKRUSTFLAGS)" $(RUSTC_CMD)
	@$(QEMU_BIN) -M $(QEMU_MACHINE) $(QEMU_ARGS) $(QEMU_DEVICES) -kernel $(KERNEL_ELF)

clippy:
	RUSTFLAGS="$(RUSTFLAGS)" $(CLIPPY_CMD)

readelf: $(KERNEL_ELF)
	readelf --headers $(KERNEL_ELF)