FROM debian:stretch-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \
    ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
    wget \
    ; \
    rm -rf /var/lib/apt/lists/*;

# Add HDF libraries
RUN apt-get update && apt-get install -y git libhdf5-serial-dev inotify-tools pkg-config libssl-dev

# Setup Rust Workspace
WORKDIR /home/rust
COPY Cargo.toml .

# Hot-reloader
COPY hot-reloader.sh  .
RUN /bin/bash -c "chmod +x hot-reloader.sh"