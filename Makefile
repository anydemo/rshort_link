.PHONY: check
check: fmt lint
	cargo check -v

.PHONY: build
build:
	cargo build -v

.PHONY: gen
gen:
	bash scripts/openapi-generator-cli.sh generate \
	-i openapi/t.yaml \
	-g rust-server \
	-o ./openapi/genapi

.PHONY: run
run:
	cargo run

.PHONY: watch
watch:
	cargo watch -x fmt -x run

.PHONY: test
test: fmt
	cargo test

.PHONY: fmt
fmt: 
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: doc
doc:
	cargo doc

.PHONY: release
release: 
	cargo build --release

.PHONY: migrate
migrate:
	diesel migration run

.PHONY: state
state:
	diesel migration list

.PHONY: ast
ast:
	cargo rustc -- -Z ast-json

.PHONY: macro_expand
macro_expand:
	cargo rustc -- -Z unstable-options --pretty=expanded

.PHONY: install_dev
install_dev:
	cargo install diesel_cli --no-default-features --features postgres
	diesel setup

.PHONY: clean
clean:
	cargo clean
