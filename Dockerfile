FROM rust:alpine

COPY . .

RUN cargo install --path .

CMD ["iot"]