SHELL := /bin/zsh

LITEX := toolchain/litex
LITEX_BOARDS := $(LITEX)/litex-boards/litex_boards
YOSYS := source ~/GreatScott/toolchain/oss-cad-suite/environment

# ulx3s 85f v3.1.7
#BOARD := "radiona_ulx3s"
#DEVICE := "LFE5U-85F"
#REVISION := "2.0"
#SDRAM := "IS42S16160"
#UART := /dev/cu.usbserial-D00137

# cynthion v0.4
#BOARD := "radiona_ulx3s"
#DEVICE := "LFE5U-25F"
#REVISION := "0.4"
#UART := /dev/cu.usbmodem22401

top: cynthion
load: load_cynthion
console: console_cynthion


# - cynthion ------------------------------------------------------------------

cynthion:
	rm -rf build
	$(YOSYS) && python -m soc.targets.gsg_cynthion \
	  --cpu-type vexriscv \
	  --cpu-variant imac \
	  --device LFE5U-12F \
	  --revision 0.4 \
	  --sys-clk-freq 60000000 \
	  --bus-standard=wishbone \
	  --integrated-rom-size=0x4000 \
	  --integrated-sram-size=0x1000 \
	  --integrated-main-ram-size=0x8000 \
	  --bios-lto \
	  --bios-console lite \
	  --csr-svd gsg_cynthion.svd \
	  --memory-x gsg_cynthion-memory.x \
	  --build

load_cynthion:
	apollo configure build/gsg_cynthion/gateware/gsg_cynthion.bit

console_cynthion:
	picocom --imap lfcrlf -b 115200 /dev/cu.usbmodem22401


# - ulx3s ---------------------------------------------------------------------

ulx3s:
	rm -rf build
	$(YOSYS) && python -m soc.targets.radiona_ulx3s \
	  --cpu-type vexriscv \
	  --cpu-variant imac \
	  --device LFE5U-85F \
	  --revision 2.0 \
	  --sdram-module IS42S16160 \
	  --csr-svd radiona_ulx3s.svd \
	  --memory-x radiona_ulx3s-memory.x \
	  --build

load_ulx3s:
	$(YOSYS) && openFPGALoader --board ulx3s build/radiona_ulx3s/gateware/radiona_ulx3s.bit

console_ulx3s:
	picocom --imap lfcrlf -b 115200 /dev/cu.usbserial-D00137


# - useful --------------------------------------------------------------------

hello:
	python -m soc.hello

clean:
	-find soc -name __pycache__ -exec rm -rf '{}' ';'
	-rm -rf build/ *.svd *.x
