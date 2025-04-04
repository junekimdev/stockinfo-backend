# Global ARG
ARG NAME

########## Stage 1: Build ##########
FROM rust:bookworm AS builder
ARG NAME
LABEL org.opencontainers.image.authors="godlyjune@gmail.com" \
      org.opencontainers.image.title="${NAME}/builder"

RUN apt-get update \
    && apt-get install -y cmake musl-tools pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ARG GIT_HASH
ENV BUILD_ID=${GIT_HASH}

# Add all files
COPY ./ ./

# Build with release profile for alpine OS
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

########## Stage 2: Runtime ##########
FROM alpine:latest
ARG NAME
ARG VERSION
LABEL org.opencontainers.image.authors="godlyjune@gmail.com" \
      org.opencontainers.image.title="${NAME}/runtime" \
      org.opencontainers.image.version=${VERSION}

WORKDIR /app

# Copy executable file from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${NAME} ./app-exe

# mount volumes
VOLUME [ "/app/config" ]

EXPOSE 4000
ENV RUST_MODE=production
CMD ["./app-exe"]
