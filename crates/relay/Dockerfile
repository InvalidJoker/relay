FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /work

FROM chef AS utils

FROM chef AS dependencies

RUN apk add --no-cache git ca-certificates musl-dev llvm21 clang21 lld21 curl --repository=https://dl-cdn.alpinelinux.org/alpine/edge/main
# zlib-dev zlib-static zstd-dev zstd-static llvm21-static llvm21-dev clang21-static
RUN ln -sf /usr/bin/clang-21 /usr/local/bin/clang \
    && ln -sf /usr/bin/ld.lld-21 /usr/local/bin/ld.lld

RUN apk upgrade --no-cache gcc --repository=https://dl-cdn.alpinelinux.org/alpine/edge/main

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM dependencies AS builder

COPY --from=planner /work/recipe.json recipe.json

RUN --mount=type=ssh cargo chef cook --release --recipe-path recipe.json --target x86_64-unknown-linux-musl

COPY . .

# build
RUN cargo build --release --target x86_64-unknown-linux-musl --bin relay

FROM scratch AS runtime

STOPSIGNAL SIGINT

# Copy the binary directly to bin directory (/bin)
COPY --from=builder /work/target/x86_64-unknown-linux-musl/release/relay /relay
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

ENTRYPOINT ["/relay"]