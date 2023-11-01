# Builder stage
# We use the latest Rust stable release as base image
FROM rust:1.68-slim AS builder
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Note: when using rust:1.72 as the baseline, the runtime failed to start due to old version of libc:
# in rust:1.72, libc:32+ is used, and in debian:bullseye-slim (2023-11-01), libc:31
# but if trying to use rust:1.63, I get the following error:
# package `actix-web v4.4.0` cannot be built because it requires rustc 1.68 or newer, while the currently active rustc version is 1.63.0
FROM debian:bullseye-slim   AS runtime
WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]