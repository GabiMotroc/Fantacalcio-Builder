FROM rustlang/rust:nightly-slim as builder

# Set the working directory
WORKDIR /usr/src/api

# Copy the source code
COPY ./api ./
COPY ./request-domain ../request-domain

ENV SQLX_OFFLINE true
ENV LLM_LS_TARGET=x86_64-unknown-linux-musl

# Build the project
RUN cargo build --release

# Use a slim image to reduce the final image size
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

FROM debian:buster-slim AS runtime
EXPOSE 8080

WORKDIR /usr/src/app

COPY --from=builder /usr/src/api/target/release/fantacalcio_builder_api .

ENTRYPOINT ["./fantacalcio_builder_api"]