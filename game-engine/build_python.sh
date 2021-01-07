echo "Building project"

cargo build --release

cp target/release/stratego-exec game-stratego
cp target/release/libstratego_lib.so ai-python
mv ai-python/libstratego_lib.so ai-python/stratego_engine.so

