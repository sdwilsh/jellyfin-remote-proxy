FROM rust:1.91-slim-bookworm@sha256:c4064394276a4afc5d80928d35b8e158936ab0267cc89fc6dd4c51a14863a71b

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install --no-install-recommends \
        cmake \
        git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
