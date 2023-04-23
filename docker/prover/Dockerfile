# syntax=docker/dockerfile:1

FROM registry.cn-beijing.aliyuncs.com/pox/rust:latest as builder

WORKDIR /target/src
COPY rust-toolchain .
RUN rustup-init -y --no-modify-path --profile minimal --default-toolchain $(cat rust-toolchain) --target $(cat /tmp/target)
# trigger fetch of crates index
RUN cargo search --limit 0

COPY . .
RUN cargo build --locked --bin prover_rpcd --release --target-dir /target --target $(cat /tmp/target) && \
      mv /target/*-unknown-linux-musl/release/prover_rpcd / && rm -rf /target

FROM registry.cn-beijing.aliyuncs.com/pox/rust:latest
ENTRYPOINT ["/prover_rpcd"]
COPY --from=builder /prover_rpcd /
