.PHONY: fmt
fmt:
	cargo fmt --all -- 

.PHONY: lint
lint:
	cargo clippy -- -D warnings


.PHONY: test
test: fmt lint
	cargo test

.PHONY: clean
clean:
	cargo clean
