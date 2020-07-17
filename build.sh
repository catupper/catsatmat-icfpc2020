#!/bin/sh

cargo vendor
cd app
cargo build --release --offline
