# Use an official Rust image as a base
FROM rust:latest AS builder

# Install GStreamer development libraries
RUN apt-get update && \
    apt-get install -y \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    pkg-config \
    gstreamer1.0-tools \
    gstreamer1.0-libav \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly

# Create a new directory for the app
WORKDIR /app

# Copy the entire app into the container
COPY . .

# Build the Rust app with release optimizations
RUN cargo build --release

# Use a minimal image for running the compiled binary
FROM debian:buster-slim

# Install necessary dependencies for running the app (GStreamer runtime and others)
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the new image
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/video-hash-server /app/video-hash-server

# Expose the port your Actix Web app will run on
EXPOSE 8080

# Run the Actix Web app
CMD ["./video-hash-server"]

