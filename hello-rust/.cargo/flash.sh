#!/usr/bin/env zsh

# exit on error
set -e

# - configuration -------------------------------------------------------------

# yosys location
YOSYS=~/GreatScott/toolchain/oss-cad-suite

# TODO can we pass target info in via build.rs or somesuch?

# ulx3s
#UART=/dev/cu.usbserial-D00137
#BASE_MEM=0x40000000
#BITSTREAM=../build/radiona_ulx3s/gateware/radiona_ulx3s.bit
#LOADER="$YOSYS/bin/openFPGALoader --board ulx3s $BITSTREAM"

# cynthion
UART=/dev/cu.usbmodem22401
BASE_MEM=0x40000000
BITSTREAM=../build/gsg_cynthion/gateware/gsg_cynthion.bit
LOADER="apollo configure $BITSTREAM 2>/dev/null"


# - run -----------------------------------------------------------------------

# create bin file
NAME=$(basename $1)
cargo objcopy --release --bin $NAME -- -Obinary $1.bin

# lxterm command
LXTERM="litex_term --kernel $1.bin --kernel-adr $BASE_MEM --speed 115200 $UART"

# configure fpga with soc bitstream
#echo "Configuring fpga: $BITSTREAM"
#eval $LOADER

# flash firmware to soc
echo "Flashing: $1.bin"
expect -c "spawn $LXTERM; send \nserialboot\n; interact"
