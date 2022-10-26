# tigerbeetle-rust
[TigerBeetle](https://github.com/tigerbeetledb/tigerbeetle) client for Rust.

## Development Setup

*Prerequisites:*
- rustc 1.62.1 (or possibly earlier, your mileage may vary)
- zig 0.9.1

```sh
# Step 1 - Pull tigerbeetle
git submodule init
git submodule update 

# Step 2 - Download zig (optional)
# NOTE: (cd ...) only changes directly for the inner command
(cd ./tigerbeetle && ./scripts/install_zig.sh)

# Step 3 - Build tb_client for your platform and use it in rust
TB_TARGET=x86_64-linux
(cd ./tigerbeetle && zig/zig build tb_client -Drelease-safe -Dtarget=$TB_TARGET)
cp ./tigerbeetle/zig-out/libtb_client.a ./pkg/native/$TB_TARGET/libtb_client.a
cp ./tigerbeetle/src/c/tb_client.h ./pkg/native/tb_client.h

# Step 3.5 - Build tigerbeetle binary for `cargo test`
(cd ./tigerbeetle && zig/zig build -Drelease-safe -Dtarget=$TB_TARGET)
cp ./tigerbeetle/zig-out/bin/tigerbeetle ./pkg/native/$TB_TARGET/tigerbeetle

# Step 4 - Build and test tigerbeetle-rust
cargo test
```

## Other clients and documentation

- [Tigerbeetle Node Client](https://github.com/tigerbeetledb/tigerbeetle-node)
- [TigerBeetle Go Client](https://github.com/tigerbeetledb/tigerbeetle-go)
