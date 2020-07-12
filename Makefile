.PHONY: docs
docs:
	cargo +nightly doc --no-deps


.PHONY: fmt
fmt:
	cargo +nightly fmt


.PHONY: install-hooks
install-hooks:
	git config --local core.hooksPath .githooks/