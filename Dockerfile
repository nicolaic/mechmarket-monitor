FROM rust:1.58.0 AS builder

RUN USER=root cargo new --bin mechmarket-monitor
WORKDIR /mechmarket-monitor

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm ./target/release/deps/mechmarket_monitor*

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./mechmarket-monitor/target/release/mechmarket-monitor .
USER 1000
ENTRYPOINT [ "./mechmarket-monitor" ]