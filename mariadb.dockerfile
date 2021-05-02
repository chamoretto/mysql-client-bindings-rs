FROM ubuntu:20.04 as base

RUN apt-get update
RUN apt-get install curl llvm-dev libclang-dev clang libmariadb3 libmariadb-dev -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain stable -y;

ARG BUILD_DIR="build_dir"
RUN set -eux; \
    . $HOME/.cargo/env; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    mkdir /$BUILD_DIR;

COPY ./src           /$BUILD_DIR/src
COPY ./build.rs      /$BUILD_DIR/
COPY ./wrapper.h     /$BUILD_DIR/
COPY ./Cargo.toml    /$BUILD_DIR/
COPY ./rustfmt.toml  /$BUILD_DIR/

ARG VERSION_FEATURE_NAME
RUN set -eux; \
    . $HOME/.cargo/env; \
    cd /$BUILD_DIR; \
    rustup component add rustfmt; \
    cargo build --release --no-default-features --features $VERSION_FEATURE_NAME;

ARG NEEDED_FILE
RUN cp $BUILD_DIR/src/$NEEDED_FILE /$BUILD_DIR/
