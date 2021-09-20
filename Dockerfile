FROM rust:slim

COPY . .

RUN cargo install --debug --path .

CMD ["iot"]