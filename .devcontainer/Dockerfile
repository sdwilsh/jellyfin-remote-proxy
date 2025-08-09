FROM rust:1.89-slim-bookworm@sha256:33219ca58c0dd38571fd3f87172b5bce2d9f3eb6f27e6e75efe12381836f71fa

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
