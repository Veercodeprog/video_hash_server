# Use an official Rust image as a base
FROM rust:latest AS builder

# Create a new directory for the app
WORKDIR /app

# Copy the entire app into the container
COPY . .

# Build the Rust app with release optimizations
RUN cargo build --release

# Use a minimal image for running the compiled binary
FROM debian:buster-slim

# Install necessary dependencies for running the app (if needed)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the new image
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/your_app_binary /app/your_app_binary

# Expose the port your Actix Web app will run on
EXPOSE 8080

# Run the Actix Web app
CMD ["./your_app_binary"]

