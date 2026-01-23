FROM rust:1.93-slim-bookworm@sha256:38d9e7c33a262bf1c58aecfbdf778205491d703a2196d4abf459e81cfe9f95e4

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
