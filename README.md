## Dependencies

    # pyenv
    curl https://pyenv.run | bash

## Rust

    rustup target add riscv32imac-unknown-none-elf

## Python Environment

    pyenv install 3.11.3
    pyenv virtualenv 3.11.3 cynthion-litex
    pyenv local cynthion-litex

    python -m pip install --upgrade pip


## Litex build dependencies

    brew install ninja
    pip install meson3


## LiteX

    wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
    chmod +x litex_setup.py
    ./litex_setup.py --init --install --config=full

    # x86_64 - intalls SiFive GCC 10.1.0-2020.08.2
    ./litex_setup.py --gcc=riscv
    export PATH=$PATH:$(echo $PWD/riscv64-*/bin/)

    # arm64 - installs GCC 11.1.0
    brew install riscv-software-src/riscv/riscv-gnu-toolchain


## Loaders

### cynthion

    # apollo
    pip install git+https://github.com/greatscottgadgets/apollo.git#egg=apollo-fpga

### ulx3s

    # ujprog
    brew install libftdi0

    cd toolchain/
    git clone https://github.com/f32c/tools.git ulx3s-tools.git
    cd ulx3s-tools.git/ujprog
    make -f Makefile.osc
