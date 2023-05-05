# syntax=docker/dockerfile:1

FROM registry.cn-beijing.aliyuncs.com/pox/rust:latest

COPY . .
RUN cargo build --release

ENTRYPOINT ["/coordinator"]
