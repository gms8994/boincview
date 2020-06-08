#!/bin/bash

## Requires inferno
## See https://docs.rs/inferno/ for more information

set -e

cargo build --release
./target/release/boincview &
bin_pid=$!

dtrace -x ustackframes=100 -n "profile-97 /pid == $bin_pid/ { @[ustack()] = count(); } tick-60s { exit(0); }"  -o out.user_stacks &
dtrace_pid=$!

sleep 30
kill $bin_pid
kill $dtrace_pid

cat out.user_stacks | inferno-collapse-dtrace > stacks.folded
cat stacks.folded | inferno-flamegraph > profile.svg

rm stacks.folded
rm out.user_stacks
