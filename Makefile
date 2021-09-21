riscv64:
	cargo build --release --features no-std --no-default-features --target riscv64gc-unknown-none-elf

x86_64:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean