FROM rustlang/rust:nightly-slim as builder

# Set the working directory
WORKDIR /usr/src/app

RUN cargo install leptosfmt
RUN cargo install trunk@0.20.3 wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

# Copy the source code
COPY ./app ./
COPY ./request-domain ../request-domain

# Build the project
RUN trunk build 

# Use a static file server image to serve the built frontend
FROM nginx:alpine

# Copy the built frontend from the builder
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Expose the port
EXPOSE 3000

# Run nginx
CMD ["nginx", "-g", "daemon off;"]