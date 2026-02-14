FROM rust:1.93-slim-bookworm@sha256:9eb3d4c46eec8564bbecb6dba21861497989df0c74743d040ea356b3615ab187

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
