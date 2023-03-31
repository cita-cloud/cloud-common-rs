#!/bin/bash

git submodule deinit --all -f
git submodule update --force --init --remote --recursive
cargo build --release --features=tonic-build
rm -rf cloud-proto/src/proto/health_check.rs
mv cloud-proto/src/proto/grpc.health.v1.rs cloud-proto/src/proto/health_check.rs
cargo fmt --all
