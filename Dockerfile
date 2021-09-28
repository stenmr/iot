# syntax=docker/dockerfile:experimental
FROM rust:slim

WORKDIR /usr/src/app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/home/root/app/target \
  cargo build

CMD ["target/release/iot"]