FROM rust:latest

WORKDIR /usr/src/weather
COPY . .

RUN cargo test -- --nocapture
