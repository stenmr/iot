# syntax=docker/dockerfile:experimental
FROM rust:slim

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/target \
  cargo install --path .

CMD ["iot"]