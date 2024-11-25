FROM rust:latest AS builder

WORKDIR /app

COPY . .
COPY crates/sites/site_1/src/web-folder web-folder
COPY crates/sites/site_1/src/web-folder/.vite web-folder/.vite
RUN ls -la web-folder

RUN cargo build -p web-server --release

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y curl wget openssl ca-certificates && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/web-server .
COPY --from=builder /app/web-folder crates/sites/site_1/src/web-folder

EXPOSE 3000

# Run the binary
CMD ["./web-server"]
