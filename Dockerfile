FROM rust:1.49.0

WORKDIR /usr/src/rust_webserver_test

COPY . .

RUN cargo build --release --locked

RUN cargo install --path .

CMD ["poke"]
