FROM rust:alpine

COPY . .

RUN cargo install --debug --path .

CMD ["iot"]