FROM rust:1.90-slim-bookworm@sha256:0a694b60da1de10034671091330d628c88af06f9ea0cc87c654d5bc4b6c7e538

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
