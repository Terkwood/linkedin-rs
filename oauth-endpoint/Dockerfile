ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust src src/
ADD --chown=rust:rust Cargo.toml Cargo.toml

#RUN cargo build --release
RUN cargo build

FROM alpine:latest
RUN apk --no-cache add ca-certificates

# CHOOSE ONE:
# debug OR release
ENV TARGET=release
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/$TARGET/linkedin-oauth-endpoint \
  /usr/local/bin/
CMD /usr/local/bin/linkedin-oauth-endpoint
