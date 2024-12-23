default: build-release

build-release: .env
	cargo build --release
	. .env/bin/activate && cd genieutils-python && maturin build --release

build-dev: .env
	cargo build
	. .env/bin/activate && cd genieutils-python && maturin build

.env:
	python3 -m venv .env
	. .env/bin/activate && python -m pip install genieutils-python/requirements-dev.txt

.PHONY: default build
