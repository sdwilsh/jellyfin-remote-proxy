FROM rust:1.94-slim-bookworm@sha256:98c4414986f091750177710b667e76c7e66c39e75da95a41d60583da16fbe47f

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
