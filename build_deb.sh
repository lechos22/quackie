#!/usr/bin/bash
cargo install cargo_deb
cargo deb --profile release --strip -o target/debian
