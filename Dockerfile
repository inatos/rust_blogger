# Build Image
FROM rust:latest as build

# Set timezone env
ENV TZ=Etc/UTC

# Add project dependencies to image
RUN apt-get update \
    && apt-get -y upgrade \
    && DEBIAN_FRONTEND="noninteractive" apt-get install -y tzdata \
    && apt-get -y install postgresql postgresql-contrib \
    && apt-get -y install libpq-dev \
    && apt-get -y install musl-tools \ 
    && apt-get -y install libssl-dev 
RUN rm -rf /var/lib/apt/lists/*
RUN cargo install diesel_cli --no-default-features --features postgres

# Set working directory to project
WORKDIR /usr/deploy/rust_blogger

# Copy project over and build it
COPY . .
RUN cargo build --release

# Point to our executable
ENTRYPOINT ./target/release/rust_blogger