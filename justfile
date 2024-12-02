#!/usr/bin/env just --justfile

zig := env_var_or_default("ZIG_BIN", "zig")

default:
    just -l

build:
    {{zig}} build

run:
    {{zig}} build run

test:
    {{zig}} build test

dbg:
    {{zig}} build
    lldb ./zig-out/bin/allot

dbgtest FILE:
    {{zig}} test --test-no-exec -femit-bin=zig-out/bin/test-exe {{FILE}}
    lldb ./zig-out/bin/test-exe

repl:
    touch repl.tmp.zig && ls repl.tmp.zig | entr -s '{{zig}} run repl.tmp.zig'

buildr:
    {{zig}} build --release=fast

runr:
    {{zig}} build run --release=fast

testr:
    {{zig}} build test --release=fast

__zig:
    ls {{zig}}
    {{zig}} version
