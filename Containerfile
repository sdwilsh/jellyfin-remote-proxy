FROM scratch as ctx
WORKDIR /
COPY Cargo.toml /
COPY Cargo.lock /
COPY src src/

FROM rust:1.92-slim-bookworm@sha256:376e6785918280aa68bef2d8d7b0204b58dfd486f370419023363c6e8cc09ec3 AS chef
WORKDIR /app
RUN --mount=type=cache,dst=/var/cache \
    --mount=type=cache,dst=/var/log \
    --mount=type=tmpfs,dst=/tmp \
    apt-get update \
    && apt-get -y install --no-install-recommends \
        cmake \
        make \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef

FROM chef as planner
COPY --from=ctx . .
RUN cargo chef prepare \
    --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY --from=ctx . .
RUN cargo build --release --bin jellyfin-remote-proxy

FROM debian:bookworm-slim@sha256:1371f816c47921a144436ca5a420122a30de85f95401752fd464d9d4e1e08271 AS runtime
# In order to run a health check, we need curl.
RUN --mount=type=cache,dst=/var/cache \
    --mount=type=cache,dst=/var/log \
    --mount=type=tmpfs,dst=/tmp \
    apt-get update \
    && apt-get -y install --no-install-recommends \
        curl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/jellyfin-remote-proxy /usr/local/bin/jellyfin-remote-proxy
ENTRYPOINT ["/usr/local/bin/jellyfin-remote-proxy"]
