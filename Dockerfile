########## Stage 1: Build ##########
FROM rust:latest AS builder

RUN apt-get update \
    && apt-get install -y cmake musl-tools pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Add all files
COPY ./ ./

# Build with release profile for alpine OS
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

########## Stage 2: Runtime ##########
FROM alpine:latest
ARG NAME
LABEL org.opencontainers.image.description="Backend for JK Stock website" \
      org.opencontainers.image.authors="godlyjune@gmail.com" \
      org.opencontainers.image.title=${NAME}

WORKDIR /app

# Copy executable file from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${NAME} ./app-exe

# mount volumes
VOLUME [ "/app/config" ]

ENV RUST_MODE=production
EXPOSE 4000
CMD ["./app-exe"]
