FROM rust:latest
RUN mkdir -p /usr/src/app/
WORKDIR /usr/src/app
RUN rustup target add wasm32-unknown-unknown
