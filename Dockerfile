FROM rust:1.42.0-alpine@sha256:fd13af547a7b00e082875567569dfa62501ebdf3468af8bfe52d08b2be109e66

WORKDIR /usr/src
COPY target/release/who_dat_kid /usr/local/bin/

CMD ["who_dat_kid"]
