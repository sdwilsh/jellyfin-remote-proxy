FROM rust:1.92-slim-bookworm@sha256:376e6785918280aa68bef2d8d7b0204b58dfd486f370419023363c6e8cc09ec3

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
