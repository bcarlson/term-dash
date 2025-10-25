# syntax=docker/dockerfile:1.4

ARG RUST_VERSION=1.75
FROM rust:${RUST_VERSION}-slim AS runtime

ARG USER_ID=1000
ARG GROUP_ID=1000
ARG USER_NAME=rustdev

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:/usr/local/rustup/toolchains/${RUST_VERSION}-x86_64-unknown-linux-gnu/bin:$PATH

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates \
        curl \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g ${GROUP_ID} ${USER_NAME} \
    && useradd -m -u ${USER_ID} -g ${GROUP_ID} ${USER_NAME}

WORKDIR /workspace

# Pre-fetch dependencies if Cargo manifest exists (optional during build).
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN if [ -f Cargo.toml ]; then cargo fetch; fi

USER ${USER_NAME}

CMD ["bash"]
