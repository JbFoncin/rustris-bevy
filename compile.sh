#!/bin/sh

# script used to compile for Linux and Windows

docker build -t rustris-compiler:local .

docker run --rm -u $UID:ubuntu -v .:/home/ubuntu/Projects/rustris-bevy rustris-compiler:local \
    cd Projects/rustris && \
    cargo build --release && \
    cargo build --release --target=x86_64-pc-windows-gnu && \
    cargo apk build --lib --release --target armv7-linux-androideabi && \
    cargo apk build --lib --release --target arm-linux-androideabi

docker image rm rustris-compiler:local -f