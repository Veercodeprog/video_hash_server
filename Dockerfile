# Use an appropriate base image
FROM ubuntu:20.04 as builder

# Install dependencies including GStreamer and FFmpeg
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-good1.0-dev \
    libgstreamer-plugins-ugly1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    libglib2.0-dev \
    libgobject-2.0-dev \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

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
EXPOSE 3000
COPY --from=planner /api-deployment-example/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /api-deployment-example/target/x86_64-unknown-linux-musl/release/api-deployment-example /api-deployment-example
ENTRYPOINT ["/api-deployment-example"]
EXPOSE 3000

