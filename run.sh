#!/usr/bin/env bash
cargo +nightly build --target=wasm32-unknown-unknown --release
ls -lh target/wasm32-unknown-unknown/release/wasm_size_bug.wasm