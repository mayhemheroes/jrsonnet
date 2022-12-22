FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD ../ /jrsonnet
WORKDIR /jrsonnet/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /jrsonnet/fuzz/target/x86_64-unknown-linux-gnu/release/jrsonnet-fuzz /