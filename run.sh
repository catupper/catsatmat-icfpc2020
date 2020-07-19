#!/bin/sh

RUST_BACKTRACE=1
/solution/target/release/app "$@" || echo "run error code: $?"
