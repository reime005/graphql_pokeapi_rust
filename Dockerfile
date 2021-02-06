FROM rust:1.49.0

WORKDIR /usr/src/rust_webserver_test

COPY . .

RUN cargo build --release --locked

RUN cargo install --path .

WORKDIR /usr/src/rust_webserver_test/pokeapi

RUN apt-get update && apt-get install -y \
  python3-pip

RUN ln -sf /usr/bin/pip3 /usr/bin/pip && \
  ln -sf /usr/bin/python3 /usr/bin/python

RUN make install

RUN make setup

RUN make serve &

RUN echo "from data.v2.build import build_all; build_all()" | python manage.py shell --settings=config.local

RUN chmod 777 /usr/src/rust_webserver_test/scripts/run.sh

CMD ["/usr/src/rust_webserver_test/scripts/run.sh"]
