FROM rust:1.91-slim-bookworm@sha256:ea63277ad2e4d89ea2e2b170ab0bde8813ed4809f66afa3db596d1ccf90b5903

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
