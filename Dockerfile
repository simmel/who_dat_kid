FROM rust:1.49.0-slim@sha256:2108dd7ec217543fa1c4be10135d0329960ce2dc5bcbc0f4f3e9c23ac17238a2 as builder
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/who_dat_kid /

EXPOSE 1337

CMD ["/who_dat_kid"]
