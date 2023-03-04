#!/bin/sh
rm perf.svg
cargo clean 
cargo build --release
sudo dtrace -c './target/release/bitcoin_address' -o out.stacks -n 'profile-997 /execname == "bitcoin_address"/ { @[ustack(100)] = count(); }'
stackcollapse out.stacks| flamegraph > perf.svg                        