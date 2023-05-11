#!/usr/bin/env python3

#
# This file is part of LiteX-Boards.
#
# Copyright (c) 2018-2019 Florent Kermarrec <florent@enjoy-digital.fr>
# Copyright (c) 2018 David Shah <dave@ds0.me>
# Copyright (c) 2023 Great Scott Gadgets <info@greatscottgadgets.com>
# SPDX-License-Identifier: BSD-2-Clause

from migen import *
from migen.genlib.resetsync import AsyncResetSynchronizer

from litex.gen import *

from litex.build.io import DDROutput

#from litex_boards.platforms import radiona_ulx3s
from soc.platforms import gsg_cynthion

from litex.soc.cores.clock import *
from litex.soc.integration.soc_core import *
from litex.soc.integration.builder import *
from litex.soc.cores.led import LedChaser
from litex.soc.cores.gpio import GPIOOut

from litedram import modules as litedram_modules
from litedram.phy import GENSDRPHY, HalfRateGENSDRPHY

# CRG ----------------------------------------------------------------------------------------------

class _CRG(LiteXModule):
    def __init__(self, platform, sys_clk_freq):
        self.rst    = Signal()
        self.cd_sys = ClockDomain()

        # # #

        # Clk / Rst
        clk60 = platform.request("clk60")
        rst   = platform.request("rst")

        # PLL
        self.pll = pll = ECP5PLL()
        self.comb += pll.reset.eq(rst | self.rst)
        pll.register_clkin(clk60, 60e6)
        pll.create_clkout(self.cd_sys, sys_clk_freq)


# BaseSoC ------------------------------------------------------------------------------------------

class BaseSoC(SoCCore):
    def __init__(self, device="LFE5U-12F", revision="0.4", toolchain="trellis", sys_clk_freq=60e6,
        with_led_chaser        = True,
        **kwargs):
        platform = gsg_cynthion.Platform(device=device, revision=revision, toolchain=toolchain)

        # CRG --------------------------------------------------------------------------------------
        self.crg = _CRG(platform, sys_clk_freq)

        # SoCCore ----------------------------------------------------------------------------------
        SoCCore.__init__(self, platform, sys_clk_freq, ident="LiteX SoC on Cynthion", **kwargs)

        # Leds -------------------------------------------------------------------------------------
        if with_led_chaser:
            self.leds = LedChaser(
                pads         = platform.request_all("user_led"),
                sys_clk_freq = sys_clk_freq)


# Build --------------------------------------------------------------------------------------------

def main():
    from litex.build.parser import LiteXArgumentParser
    parser = LiteXArgumentParser(platform=gsg_cynthion.Platform, description="LiteX SoC on Cynthion")
    parser.add_target_argument("--device",          default="LFE5U-12F",      help="FPGA device (LFE5U-12F, LFE5U-25F).")
    parser.add_target_argument("--revision",        default="0.4",            help="Board revision (0.4 or 0.7).")
    parser.add_target_argument("--sys-clk-freq",    default=60e6, type=float, help="System clock frequency.")
    args = parser.parse_args()

    soc = BaseSoC(
        device                 = args.device,
        revision               = args.revision,
        toolchain              = args.toolchain,
        sys_clk_freq           = args.sys_clk_freq,
        **parser.soc_argdict)

    builder = Builder(soc, **parser.builder_argdict)
    if args.build:
        builder.build(**parser.toolchain_argdict)

    if args.load:
        prog = soc.platform.create_programmer()
        prog.load_bitstream(builder.get_bitstream_filename(mode="sram", ext=".svf")) # FIXME

if __name__ == "__main__":
    main()
