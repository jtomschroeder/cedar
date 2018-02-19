#!/bin/bash

set -e

cargo +nightly build --target=wasm32-unknown-unknown --release --example buttons
cp target/wasm32-unknown-unknown/release/examples/buttons.wasm lib/wasm/.
