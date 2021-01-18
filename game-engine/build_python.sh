echo "Building project"

cargo build --release

cp target/release/stratego-exec game-stratego
cp target/release/libstratego_lib.so ai_python/src
mv ai_python/src/libstratego_lib.so ai_python/src/stratego_engine.so
