
test:
	cargo test

run-number-swap:
	cd number-swap && cargo test

run-static-vec:
	cd static_vec && cargo test

help:
	@echo "run-number-swap: run cargo test inside cd number-swap"
	@echo "run-static-vec: run cargo test inside cd static_vec"
	@echo "test: run cargo test"