#!/bin/sh

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run --features=pcmp -O -a fuzz_target_1 -- -only_ascii=1 -max_len=5000 "$@"
