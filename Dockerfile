# Use an official Rust image as a base
FROM rust:latest AS builder

# Install GStreamer, FFmpeg, and other required development libraries
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libavfilter-dev \
    libavformat-dev \
    libavcodec-dev \
    libavdevice-dev \
    libavutil-dev \
    libswscale-dev \
    libunwind-dev \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    libgstreamer-plugins-good1.0-dev \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-tools \
    gstreamer1.0-x \
    gstreamer1.0-alsa \
    gstreamer1.0-gl \
    gstreamer1.0-gtk3 \
    gstreamer1.0-qt5 \
    gstreamer1.0-pulseaudio

# Create a new directory for the app
WORKDIR /app

# Copy the entire app into the container
COPY . .

# Set PKG_CONFIG_PATH for FFmpeg and GStreamer
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Build the Rust app with release optimizations
RUN cargo build --release

# Use a minimal image for running the compiled binary
FROM debian:buster-slim

# Install necessary runtime dependencies (GStreamer, FFmpeg, and others)
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav \
    gstreamer1.0-tools \
    gstreamer1.0-x \
    gstreamer1.0-alsa \
    gstreamer1.0-gl \
    gstreamer1.0-gtk3 \
    gstreamer1.0-qt5 \
    gstreamer1.0-pulseaudio \
    ffmpeg && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the new image
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/video-hash-server /app/video-hash-server

# Expose the port your Actix Web app will run on
EXPOSE 8080

# Run the Actix Web app
CMD ["./video-hash-server"]

