#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters" && exit 1
fi

APP=${1}

export RUST_BACKTRACE=1

cargo +nightly build --target=wasm32-unknown-unknown --release --example ${APP}
cp target/wasm32-unknown-unknown/release/examples/${APP}.wasm lib/wasm/code.wasm
