CARGO ?= cargo

.PHONY: build test

build:
	$(CARGO) build

test:
	$(CARGO) test
