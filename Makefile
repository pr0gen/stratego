# === Stratego lib ===
engine:
		@echo "== Build Stratego engine =="
		cargo build --release --manifest-path game-engine/Cargo.toml
		@echo "Copying stratego executable "
		cp game-engine/target/release/stratego-exec game-engine/game-stratego
		@echo "Copying stratego library in python project"
		cp game-engine/target/release/libstratego_lib.so game-engine/ai_python/src
		mv game-engine/ai_python/src/libstratego_lib.so game-engine/ai_python/src/stratego_engine.so

rust-test:
	@echo "== Cargo test =="
	cargo test --manifest-path game-engine/Cargo.toml

rust-clippy:
	@echo "== Cargo clippy =="
	cargo clippy --manifest-path game-engine/Cargo.toml

python-test:
	@echo "== Pytest =="
	pytest

api-build:
	@echo "== Run Stratego API=="
	uvicorn api:app --reload --app-dir 'game-engine'

commit: engine rust-test rust-clippy python-test

lib-build: engine rust-test python-test
