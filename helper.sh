#!/bin/bash
set -e

su iphands -c "RUSTFLAGS='-C target-cpu=znver2' cargo build --release && \
cp ./target/release/niced ./target/release/niced.nostrip && \
strip -s ./target/release/niced"

# su iphands -c "RUST_BACKTRACE=1 cargo run --bin niced"

ls -lh ./target/release/
time ./target/release/niced
