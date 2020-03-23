FROM rust:1.42.0-alpine@sha256:fd13af547a7b

WORKDIR /usr/src
COPY . .

RUN cargo build --release

CMD ["target/release/who_dat_kid"]
