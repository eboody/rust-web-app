FROM rust:latest AS builder

WORKDIR /app

COPY . .

# ENV RUST_MIN_STACK=934217728
# # ENV RUST_BACKTRACE=1
# ENV CARGO_BUILD_JOBS=1

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
COPY --from=builder /app/web-folder ./web-folder
# Copy the contents of the hidden .vite directory to /web-folder in the final image
COPY --from=builder /app/js/.vite /web-folder/js/.vite

EXPOSE 3000

# Run the binary
CMD ["./web-server"]
