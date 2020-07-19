#!/bin/sh

RUST_LOG=info RUST_BACKTRACE=1 /solution/target/release/app "$@" || echo "run error code: $?"
