FROM rustlang/rust:nightly-slim as builder

# Set the working directory
WORKDIR /usr/src/api

# Copy the source code
COPY ./api ./
COPY ./request-domain ../request-domain

ENV SQLX_OFFLINE true
ENV LLM_LS_TARGET=x86_64-unknown-linux-musl

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools

# Build the project for the musl target
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a slim image to reduce the final image size
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

FROM debian:bullseye-slim AS runtime
EXPOSE 3000

WORKDIR /usr/src/app

RUN apt install libc6

#COPY --from=builder /usr/src/api/target/release/fantacalcio_builder_api .
COPY --from=builder /usr/src/api/target/x86_64-unknown-linux-musl/release/fantacalcio_builder_api .

ENTRYPOINT ["./fantacalcio_builder_api"]