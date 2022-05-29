#!/bin/sh

set -e

cd $(dirname $0)/..

bundler install
cargo build --release --locked

export PATH=$PWD/target/release:$PWD/tools:$PATH
export EIN_ROOT=$PWD

bundler exec cucumber --publish-quiet -e memory_leak
