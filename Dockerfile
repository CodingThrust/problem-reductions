# Generated for Smithery deployment. See: https://smithery.ai/docs/config#dockerfile
# Multi-stage build: compile the Rust binary, then copy into a slim runtime image.

FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependency builds: copy manifests first, then build deps only
COPY Cargo.toml Cargo.lock ./
COPY src/lib.rs src/lib.rs
COPY problemreductions-cli/Cargo.toml problemreductions-cli/Cargo.toml
RUN mkdir -p problemreductions-cli/src && echo 'fn main() {}' > problemreductions-cli/src/main.rs
RUN cargo build --release -p problemreductions-cli 2>/dev/null || true

# Copy full source and build for real
COPY . .
RUN cargo build --release -p problemreductions-cli

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/pred /usr/local/bin/pred

ENTRYPOINT ["pred", "mcp"]
