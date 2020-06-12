# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
#RUN cargo build --release
RUN cargo build

# Now, we need to build our _real_ Docker container, copying in `linkedin-oauth-endpoint`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/linkedin-oauth-endpoint \
  /usr/local/bin/
CMD /usr/local/bin/linkedin-oauth-endpoint
