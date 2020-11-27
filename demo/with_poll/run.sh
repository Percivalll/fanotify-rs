#!/bin/sh
cargo build --release
mkdir tmp 2>/dev/null
sudo mount -t tmpfs tmpfs tmp
sudo ./target/release/with_poll `pwd`/tmp $1
