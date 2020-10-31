#!/bin/bash
set -e

su iphands -c "RUSTFLAGS='-C target-cpu=znver2' cargo build --release"
ls -lh ./target/release/
time ./target/release/niced
