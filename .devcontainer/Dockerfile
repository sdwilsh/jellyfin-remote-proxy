FROM rust:1.89-slim-bookworm@sha256:21e2ac30e72a6d5b6d667b573eadad0578be9a5a99bac0b2b99b3d37795f90f9

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
