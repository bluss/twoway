#!/bin/sh

DIR=$(dirname "$0")
V=$(cat "$DIR"/nightly-version)
cargo +$V fuzz run -O fuzz_target_1 -- -only_ascii=1 -max_len=512 "$@"
