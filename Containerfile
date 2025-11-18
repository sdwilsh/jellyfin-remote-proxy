FROM scratch as ctx
WORKDIR /
COPY Cargo.toml /
COPY Cargo.lock /
COPY src src/

FROM rust:1.91-slim-bookworm@sha256:ea63277ad2e4d89ea2e2b170ab0bde8813ed4809f66afa3db596d1ccf90b5903 AS chef
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

FROM debian:bookworm-slim@sha256:b4aa902587c2e61ce789849cb54c332b0400fe27b1ee33af4669e1f7e7c3e22f AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/jellyfin-remote-proxy /usr/local/bin/jellyfin-remote-proxy
ENTRYPOINT ["/usr/local/bin/jellyfin-remote-proxy"]
