# Use the official Rust image from Docker Hub as a base
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/RustEmbedBot

# Copy the entire project into the container
COPY . .

# Build the project
RUN cargo build --release

# Specify the entry point
CMD ["./target/release/RustEmbedBot"]
