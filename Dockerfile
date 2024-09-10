# Use an appropriate base image
FROM ubuntu:20.04 as builder

# Install dependencies including GStreamer and FFmpeg
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
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
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Debug pkg-config
RUN pkg-config --libs --cflags gstreamer-1.0 || echo "GStreamer not found"

# Set up Rust and the workspace
FROM rust:latest as rust

# Install cargo-chef for caching dependencies
RUN cargo install cargo-chef

# Set working directory
WORKDIR /api-deployment-example

# Copy source code
COPY . .

# Prepare and build
FROM builder as chef
COPY --from=rust /usr/local/cargo/bin/cargo-chef /usr/local/bin/
RUN cargo chef prepare --recipe-path recipe.json

FROM builder as planner
COPY --from=chef /api-deployment-example/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Final build stage
FROM rust as final
COPY --from=planner /api-deployment-example/target/x86_64-unknown-linux-musl/release/api-deployment-example /api-deployment-example
ENTRYPOINT ["/api-deployment-example"]
EXPOSE 8080

