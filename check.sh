echo "old solver"
RUSTFLAGS="-Znext-solver=coherence" cargo +nightly clean -p bilrost
RUSTFLAGS="-Znext-solver=coherence" cargo +nightly check
echo "new solver"
cargo +nightly clean -p bilrost
cargo +nightly check
