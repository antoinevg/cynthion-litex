#
# This file is part of LiteX-Boards.
#
# Copyright (c) 2018-2019 Florent Kermarrec <florent@enjoy-digital.fr>
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-2-Clause

from litex.build.generic_platform import *
from litex.build.lattice import LatticeECP5Platform
from litex.build.lattice.programmer import UJProg

# IOs ----------------------------------------------------------------------------------------------

_io_common = [
    # Clk / Rst
    ("clk60", 0, Pins("A8"), IOStandard("LVCMOS33")),
    ("rst",   0, Pins("D3"), IOStandard("LVCMOS33")), # user_io 1

    # User I/O
    #("user_io", 0, Pins( "C3"), IOStandard("LVCMOS33"), Misc("PULLMODE=DOWN")),
    #("user_io", 1, Pins( "D3"), IOStandard("LVCMOS33"), Misc("PULLMODE=DOWN")),

    # Leds
    ("user_led", 0, Pins("P14"), IOStandard("LVCMOS33")),
    ("user_led", 1, Pins("P16"), IOStandard("LVCMOS33")),
    ("user_led", 2, Pins("P15"), IOStandard("LVCMOS33")),
    ("user_led", 3, Pins("R16"), IOStandard("LVCMOS33")),
    ("user_led", 4, Pins("R15"), IOStandard("LVCMOS33")),
    ("user_led", 5, Pins("T15"), IOStandard("LVCMOS33")),

    # Serial
    ("serial", 0,
        Subsignal("rx", Pins("R14"), IOStandard("LVCMOS33")),
        Subsignal("tx", Pins("T14"), IOStandard("LVCMOS33"), Misc("PULLMODE=UP")),
    ),
]


# Platform -----------------------------------------------------------------------------------------

class Platform(LatticeECP5Platform):
    default_clk_name   = "clk60"
    default_clk_period = 1e9/60e6

    def __init__(self, device="LFE5U-12F", revision="0.4", toolchain="trellis", **kwargs):
        assert device in ["LFE5U-12F", "LFE5U-25F"]
        assert revision in ["0.4", "0.7"]
        _io = _io_common
        LatticeECP5Platform.__init__(self, device + "-6BG256C", _io, toolchain=toolchain, **kwargs)

    def create_programmer(self):
        return UJProg()

    def do_finalize(self, fragment):
        LatticeECP5Platform.do_finalize(self, fragment)
        self.add_period_constraint(self.lookup_request("clk60", loose=True), 1e9/60e6)
