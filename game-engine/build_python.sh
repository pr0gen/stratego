echo "Building project"

cargo build --release

cp target/release/libgame_engine.so python
mv python/libgame_engine.so python/stratego_engine.so

echo "Project builded"

