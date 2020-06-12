FROM debian:buster AS builder

RUN apt-get update && apt-get install -y curl build-essential


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM debian:buster

RUN apt-get update

COPY --from=builder \
  /target/release/linkedin-oauth-endpoint \
  /usr/local/bin/

WORKDIR /root
ENV RUST_LOG=info
CMD ROCKET_PORT=$PORT /usr/local/bin/linkedin-oauth-endpoint
