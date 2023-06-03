cd test-crate
cargo update -q
cargo build -q
cd ..
mdbook test -L test-crate/target/debug/deps/
