FROM rust:1.45.2-slim-stretch@sha256:dc0f86106ce18a2bd2ab706eeb163f1867435b94ba32a070984ba103126b79f6 as builder
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/who_dat_kid /

EXPOSE 1337

CMD ["/who_dat_kid"]
