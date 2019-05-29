.PHONY: test

test:
	./maybe_start_solace.sh
	sh clean.sh examples/config.yaml testvpn
	cargo test
	sh clean.sh examples/config.yaml testvpn