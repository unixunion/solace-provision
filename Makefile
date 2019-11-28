.PHONY: test

test:
	./maybe_start_solace.sh
	RUST_BACKTRACE=full  RUST_LOG=solace_provision  cargo test --  --test-threads=1
