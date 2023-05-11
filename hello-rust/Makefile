# - helpful -------------------------------------------------------------------

#BIN=firmware
#TARGET=target/riscv32imac-unknown-none-elf

BIN=hello-rust
TARGET=target/riscv32i-unknown-none-elf/release

size:
	cargo size --release --bin $(BIN) -- -A
	cargo size --release --bin $(BIN) -- -B

stack:
	@# cargo size --release --bin $(BIN) -- -A | grep stack # 12504
	cargo size --release --bin $(BIN) -- -A | grep stack # 41576 / 40240

headers:
	cargo readobj --release --bin $(BIN) -- --file-headers

disasm:
	riscv64-unknown-elf-objdump --disassemble --demangle $(TARGET)/$(BIN)
	@# cargo objdump --release --bin $(BIN) -- --disassemble --no-show-raw-insn --print-imm-hex
	@# cargo objdump --release --bin $(BIN) -- --disassemble

disasm-src:
	riscv64-unknown-elf-objdump --disassemble --demangle --source $(TARGET)/$(BIN)
	@# cargo objdump --release --bin $(BIN) -- --disassemble --source --no-show-raw-insn --print-imm-hex
	@# cargo objdump --release --bin $(BIN) -- --disassemble  --source

syms:
	cargo objdump --release --bin $(BIN) -- --syms

rodata:
	cargo objdump --release --bin $(BIN) -- --full-contents --section .rodata

data:
	cargo objdump --release --bin $(BIN) -- --full-contents --section .data

nm:
	@# cargo nm --release --bin $(BIN) -- --debug-syms --format=bsd --print-size --size-sort --radix=d
	@# cargo nm --release --bin $(BIN) -- --size-sort --radix=d | grep -v Lanon | grep -v 00000000
	cargo nm --release --bin $(BIN) -- --size-sort | grep -v Lanon | grep -v 00000000

elf-size-analyze:
	@# https://github.com/jedrzejboczar/elf-size-analyze
	elf-size-analyze -t riscv64-unknown-elf- \
		--human-readable --alternating-colors --max-width 240 \
		--ram --rom --print-sections \
		$(TARGET)/$(BIN)

elf-ram:
	@# https://github.com/jedrzejboczar/elf-size-analyze
	elf-size-analyze -t riscv64-unknown-elf- \
		--human-readable --alternating-colors --max-width 240 \
		--ram \
		$(TARGET)/$(BIN)

elf-rom:
	@# https://github.com/jedrzejboczar/elf-size-analyze
	elf-size-analyze -t riscv64-unknown-elf- \
		--human-readable --alternating-colors --max-width 240 \
		--rom \
		$(TARGET)/$(BIN)

elf-sections:
	elf-size-analyze -t riscv64-unknown-elf- \
		--human-readable \
		--print-sections \
		$(TARGET)/$(BIN)

elf-text:
	elf-size-analyze -t riscv64-unknown-elf- \
		--human-readable \
		-S2 \
		$(TARGET)/$(BIN)
