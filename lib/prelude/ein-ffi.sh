#!/bin/sh

set -e

cd ffi
cargo build --release
echo ffi/target/release/libprelude.a
