# Builder stage
# We use the latest Rust stable release as base image
FROM lukemathwalker/cargo-chef:latest-rust-1.68.0 as chef
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release  --bin zero2prod

# Note: when using rust:1.72 as the baseline, the runtime failed to start due to old version of libc:
# in rust:1.72, libc:32+ is used, and in debian:bullseye-slim (2023-11-01), libc:31
# but if trying to use rust:1.63, I get the following error:
# package `actix-web v4.4.0` cannot be built because it requires rustc 1.68 or newer, while the currently active rustc version is 1.63.0
FROM debian:bullseye-slim   AS runtime
WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]