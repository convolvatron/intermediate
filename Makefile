BINARY=target/debug/i1
KERNEL=./moss-kernel/target/aarch64-unknown-none-softfloat/debug/moss

run: $(BINARY) $(KERNEL)
	$(BINARY) $(KERNEL)

force:

$(KERNEL): force
	cd moss-kernel ; cargo build

$(BINARY): src/main.rs
	(cd xhypervisor ; cargo build)
	cargo +nightly build
	codesign --entitlements xhypervisor/app.entitlements --force -s - $(BINARY)

clean:
	rm $(BINARY)
