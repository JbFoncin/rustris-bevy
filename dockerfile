FROM ubuntu:resolute-20260413

USER root

RUN apt-get update && \
    apt-get install build-essential -y && \
    apt-get install curl -y && \
    apt-get install mingw-w64 -y && \
    apt install android-sdk -y && \
    apt install sdkmanager -y && \
    apt update && \
    sdkmanager --install "ndk;r29" --sdk_root=/usr/lib/android-sdk/ && \
    yes | sdkmanager --licenses && \
    apt-get install libssl-dev -y && \
    apt install pkg-config -y && \
    apt install strace -y && \
    sdkmanager --install "build-tools;29.0.3" --sdk_root=/usr/lib/android-sdk/ && \
    sdkmanager --install "platforms;android-34" --sdk_root=/usr/lib/android-sdk/

ENV HOME=/home/ubuntu/ 

RUN mkdir $HOME/Projects

USER ubuntu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.98.1 -y

ENV PATH=$HOME".cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add arm-linux-androideabi && \
    rustup target add armv7-linux-androideabi && \
    cargo install cargo-apk

ENV ANDROID_HOME=/opt/android-sdk/ ANDROID_NDK_HOME=/opt/android-sdk/ndk/29.0.14206865/ NDK_HOME=/opt/android-sdk/build-tools/29.0.3 \
    PATH=$PATH:/usr/lib/android-sdk/platform-tools


WORKDIR $HOME/Projects/rustris-bevy