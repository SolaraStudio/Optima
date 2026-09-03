FROM rust:1.80-bookworm AS builder
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN apt-get update && apt-get install -y \
    libasound2-dev libudev-dev libjack-dev pkg-config \
    libssl-dev libfontconfig1-dev libfreetype6-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release --lib

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libfontconfig1 libfreetype6 libasound2 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/liboptima.so /usr/local/lib/
ENV LD_LIBRARY_PATH=/usr/local/lib
CMD ["/bin/bash"]
