.PHONY: docs
docs:
	cargo +nightly doc --no-deps

.PHONY: odocs
odocs:
	cargo +nightly doc --no-deps --open

.PHONY: fmt
fmt:
	cargo +nightly fmt --all

.PHONY: install-hooks
install-hooks:
	git config --local core.hooksPath .githooks/

.PHONY: loc
loc:
	scc -i rs,json,md