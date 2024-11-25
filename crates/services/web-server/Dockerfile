FROM rust:latest AS builder

WORKDIR /app

# Cache dependency installation by copying only Cargo files first
COPY Cargo.toml Cargo.lock ./

# Pre-fetch dependencies
RUN cargo fetch

COPY . .

# Ensure `web-folder` and its subdirectories are in the correct location
COPY crates/sites/site_1/src/web-folder web-folder

# Verify contents for debugging purposes
RUN ls -la web-folder

RUN cargo build -p web-server --release

# Use a smaller base image for the final stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && \
  apt-get install -y \
    curl wget openssl ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/web-server .

# Copy the `web-folder` assets
COPY --from=builder /app/web-folder crates/sites/site_1/src/web-folder

EXPOSE 3000

CMD ["./web-server"]
