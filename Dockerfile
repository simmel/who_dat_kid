FROM rust:1.42.0-slim@sha256:ec3346251d33118a9c43217bdd76563699dfda9a1cfd9c6358253630514ac763 as builder
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM rust:1.42.0-alpine@sha256:fd13af547a7b00e082875567569dfa62501ebdf3468af8bfe52d08b2be109e66

COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/who_dat_kid /usr/local/bin/

CMD ["who_dat_kid"]
