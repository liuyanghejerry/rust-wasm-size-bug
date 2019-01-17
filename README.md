This repository is used for reproducing a size issue when compling code to WASM.

To get things compiled, you'll need nightly toolchain and target "wasm32-unknown-unknown", which can be installed as follow:

```shell
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
```

Then you can run following command to compile code:

```shell
cargo +nightly build --target=wasm32-unknown-unknown --release
```

Or use shell script:

```shell
./run.sh
```

The WASM file is located at `target/wasm32-unknown-unknown/release/wasm_size_bug.wasm`.

### `rustc 1.28.0-nightly (2a1c4eec4 2018-06-25)`
In my macOS with `rustc 1.28.0-nightly (2a1c4eec4 2018-06-25)`, the binary is about 639K.

### `rustc 1.31.0-nightly (4efdc04a5 2018-10-06)`
With LTO enabled, it's 52KB now.

### `rustc 1.33.0-nightly (ceb251214 2019-01-16)`
It's 63KB now. No idea why it is going backwards.
