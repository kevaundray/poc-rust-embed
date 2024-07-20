FROM rust:1.78.0-bullseye AS builder

COPY . demo

RUN cd demo && cargo install --debug --path .

FROM ubuntu:22.04

COPY --from=builder /usr/local/cargo/bin/poc /usr/local/bin/poc

ENTRYPOINT ["poc"]