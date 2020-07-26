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

.PHONY: docker.release
docker.release:
	env DOCKER_BUILDKIT=1 BUILDKIT_PROGRESS=plain docker build -t alexkreidler/osprey -f ./docker/release.Dockerfile .

.PHONY: docker.server
docker.server:
	env DOCKER_BUILDKIT=1 BUILDKIT_PROGRESS=plain docker build -t alexkreidler/osprey-server -f ./docker/server.Dockerfile .


.PHONY: docker
docker: docker.server docker.release