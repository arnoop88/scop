CARGO = cargo
TARGET = target/debug/scop

all: build

build:
	$(CARGO) build

run: build
	$(TARGET)

clean:
	$(CARGO) clean

release:
	$(CARGO) build --release

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy

.PHONY: all build run clean release test fmt clippy
