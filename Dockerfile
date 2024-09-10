# Use an official base image
FROM ubuntu:20.04

# Set environment variables to non-interactive (for non-interactive installation)
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    wget \
    git \
    pkg-config \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libgstreamer-plugins-good1.0-dev \
    libgstreamer-plugins-bad1.0-dev \
    libgstreamer-plugins-ugly1.0-dev \
    gstreamer1.0-tools \
    gstreamer1.0-libav \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-qt5 \
    python3-gst-1.0 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Optional: Install additional GStreamer plugins
RUN apt-get update && \
    apt-get install -y \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-rtsp \
    gstreamer1.0-pipewire \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace

# Optionally, copy your application code into the container
# COPY . /workspace

# Command to run when the container starts
CMD ["bash"]

