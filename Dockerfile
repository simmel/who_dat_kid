FROM rust:1.42.0-slim@sha256:ec3346251d33118a9c43217bdd76563699dfda9a1cfd9c6358253630514ac763 as builder
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/who_dat_kid /

EXPOSE 1337

CMD ["/who_dat_kid"]
