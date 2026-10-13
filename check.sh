echo "old solver"
# RUSTFLAGS="-Znext-solver=coherence" cargo +nightly clean -p bilrost
# RUSTFLAGS="-Znext-solver=coherence" cargo +nightly check
time rustc +nightly --edition 2021 -Znext-solver=coherence --crate-type=lib src/lib.rs
echo "new solver"
# cargo +nightly clean -p bilrost
# cargo +nightly check
time rustc +nightly --edition 2021 --crate-type=lib src/lib.rs
