SHELL := /bin/zsh

top: cynthion
load: load_cynthion
console: console_cynthion

# - configuration -------------------------------------------------------------

LITEX := toolchain/litex
LITEX_BOARDS := $(LITEX)/litex-boards/litex_boards

YOSYS := source ~/GreatScott/toolchain/oss-cad-suite/environment

# variants: imac lite minimal
SOC_CONFIG := --cpu-type vexriscv \
			  --cpu-variant imac \
			  --bus-standard=wishbone \
			  --integrated-rom-size=0x4000 \
			  --integrated-sram-size=0x1000 \
			  --integrated-main-ram-size=0x8000 \
			  --bios-lto \
			  --bios-console lite


# - cynthion ------------------------------------------------------------------

cynthion:
	rm -rf build
	$(YOSYS) && python -m soc.targets.gsg_cynthion \
	  $(SOC_CONFIG) \
	  --device LFE5U-12F \
	  --revision 0.4 \
	  --sys-clk-freq 60000000 \
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


# - simulation ----------------------------------------------------------------

#SIM_BIN := demo.bin
SIM_BIN := ../hello-rust/hello-rust.bin

# 1 microsecond = 1_000_000			: 1000000
# 1 millisecond = 1_000_000_000		: 1000000000
# 1 second		= 1_000_000_000_000 : 1000000000000

# just before serialboot: ~ 520ms
SIM_START	 := 520000000000
# then run for: 20ms
SIM_DURATION := 20000000000

# calculate SIM_END
SIM_END := $$(( $(SIM_START) + $(SIM_DURATION) ))

prepsim:
	-rm -rf simulation/*
	cd simulation && litex_sim $(SOC_CONFIG) \
	  --no-compile-gateware \
	  --gtkwave-savefile
	cd simulation && litex_bare_metal_demo --build-path=build/sim/

sim:
	cd simulation && litex_sim $(SOC_CONFIG) \
	  --ram-init=$(SIM_BIN) \
	  --trace --trace-start $(SIM_START) --trace-end $(SIM_END) \
	  --gtkwave-savefile


gtkwave:
	open simulation/build/sim/gateware/sim.gtkw

# - useful --------------------------------------------------------------------

hello:
	python -m soc.hello

clean:
	-find soc -name __pycache__ -exec rm -rf '{}' ';'
	-rm -rf build/ *.svd *.x
