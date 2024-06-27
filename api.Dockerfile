FROM rustlang/rust:nightly-slim as builder

# Set the working directory
WORKDIR /usr/src/api

# Copy the source code
COPY ./api ./
COPY ./request-domain ../request-domain

ENV SQLX_OFFLINE true

# Build the project
RUN cargo build --release

# Use a slim image to reduce the final image size
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

FROM debian:buster-slim AS runtime
EXPOSE 8080
COPY --from=builder /usr/src/api/target/release /usr/local/bin
ENTRYPOINT ["/usr/local/bin"]