FROM rust:1.91-slim-bookworm@sha256:6f125d5c9657a0e0c93785f57350a1a79a1671e973d9b59afce6442827f73f86

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
