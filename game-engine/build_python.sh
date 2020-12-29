echo "Building project"

cargo build --release

cp target/release/libgame_engine.so ai-python
mv ai-python/libgame_engine.so ai-python/stratego_engine.so

echo "Project builded"

