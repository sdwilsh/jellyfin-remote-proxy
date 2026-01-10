FROM rust:1.92-slim-bookworm@sha256:a8ce22642819f5f54d37e025f7020b51dcbac39e5d0ea907a63eda8b03458de5

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
