# Base Dockerfile for Nyxora

FROM rust:1.80-bullseye

# Install Python and other dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    pkg-config \
    libssl-dev \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Qiskit for quantum simulations
RUN pip3 install qiskit

WORKDIR /app

# Copy the project files
COPY . .

# Build the project
RUN cargo build --release

# Default command
CMD ["./target/release/nyxora-node"]