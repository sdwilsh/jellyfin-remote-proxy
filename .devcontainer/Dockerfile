FROM rust:1.90-slim-bookworm@sha256:3bee83bb39aff63f1c269a276f3dfc6e65f4251c2d7cc73e6181d8d3aca62e03

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
