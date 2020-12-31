FROM rust:latest

WORKDIR /usr/src/weather
COPY . .
EXPOSE 3030

RUN cargo build

CMD ["cargo run"]