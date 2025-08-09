FROM scratch as ctx
WORKDIR /
COPY Cargo.toml /
COPY Cargo.lock /
COPY src src/

FROM rust:1.89-slim-bookworm@sha256:33219ca58c0dd38571fd3f87172b5bce2d9f3eb6f27e6e75efe12381836f71fa AS chef
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

FROM debian:bookworm-slim@sha256:2424c1850714a4d94666ec928e24d86de958646737b1d113f5b2207be44d37d8 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/jellyfin-remote-proxy /usr/local/bin/jellyfin-remote-proxy
ENTRYPOINT ["/usr/local/bin/jellyfin-remote-proxy"]
