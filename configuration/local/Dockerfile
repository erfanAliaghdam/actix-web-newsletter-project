FROM rust:latest

RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

RUN mkdir src && mkdir tests && mkdir migrations


COPY ./src ./src
COPY ./tests ./tests
COPY ./migrations ./migrations


RUN cargo install cargo-watch
RUN cargo install sqlx-cli

EXPOSE 8000
