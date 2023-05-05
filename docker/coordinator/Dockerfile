# syntax=docker/dockerfile:1

FROM registry.cn-beijing.aliyuncs.com/pox/rust:latest
WORKDIR /target/src
COPY rust-toolchain .

COPY . .
RUN cargo build --release

ENTRYPOINT ["/coordinator"]
