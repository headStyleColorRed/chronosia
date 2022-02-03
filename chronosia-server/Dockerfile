# Use the Rust official image
FROM rust:latest

# Move workspace
WORKDIR /home

# Copy the files in your machine to the Docker image
COPY ./scripts ./scripts
COPY ./src ./src
COPY ./docker.env .
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./diesel.toml .
COPY ./Makefile.toml .

# Set correct environment
RUN mv docker.env .env

RUN cargo build --release

EXPOSE 8080

# Run the binary
CMD ["./target/release/chronosia"]