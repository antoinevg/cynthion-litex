#
# This file is part of LUNA.
#


# - configuration -------------------------------------------------------------

UART=/dev/cu.usbmodem22401
BASE_MEM = 0x40000000

TARGET	= main

CROSS  ?= riscv64-unknown-elf-

CC		= $(CROSS)gcc
OBJCOPY = $(CROSS)objcopy

CFLAGS	= -march=rv32imac -mabi=ilp32 -g -Os
LDFLAGS = -Tlink.ld	 -nostdlib


# - build ---------------------------------------------------------------------

SOURCES = \
	start.S \
	main.c

# By default, build our binary.
all: clean $(TARGET).bin

#
# Firmware binary.
#

$(TARGET).elf: $(SOURCES)
	$(CC) $(CFLAGS) $(LDFLAGS) $(SOURCES) -o $@

$(TARGET).bin: $(TARGET).elf
	$(OBJCOPY) -O binary $< $@


#
# Virtual/command targets.
#

.PHONY: clean program

clean:
	rm -f $(TARGET).elf $(TARGET).bin


# - run -----------------------------------------------------------------------

LXTERM=litex_term --kernel main.bin --kernel-adr $(BASE_MEM) --speed 115200 $(UART)

# Loads our "Hello world" program onto the FPGA.
run: $(TARGET).bin $(SOC)
	echo "Flashing: $1.bin"
	expect -c "spawn $(LXTERM); send \nserialboot\n; interact"



# - helpful -------------------------------------------------------------------

BIN=main

disasm:
	riscv64-unknown-elf-objdump --disassemble --demangle $(BIN).elf

disasm-src:
	riscv64-unknown-elf-objdump --disassemble --demangle --source $(BIN).elf
