FROM messense/rust-musl-cross:x86_64-musl as chef

# Install GStreamer and its plugins
RUN apt-get update && \
    apt-get install -y \
    gstreamer1.0-tools \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-plugins-bad

# Install cargo-chef
RUN cargo install cargo-chef
WORKDIR /api-deployment-example

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
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

