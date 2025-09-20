FROM scratch as ctx
WORKDIR /
COPY Cargo.toml /
COPY Cargo.lock /
COPY src src/

FROM rust:1.90-slim-bookworm@sha256:0a694b60da1de10034671091330d628c88af06f9ea0cc87c654d5bc4b6c7e538 AS chef
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

FROM debian:bookworm-slim@sha256:df52e55e3361a81ac1bead266f3373ee55d29aa50cf0975d440c2be3483d8ed3 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/jellyfin-remote-proxy /usr/local/bin/jellyfin-remote-proxy
ENTRYPOINT ["/usr/local/bin/jellyfin-remote-proxy"]
