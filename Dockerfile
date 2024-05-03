# based on latest rust bookworm image
FROM rust:bookworm

LABEL maintainer="June Kim" version="1.0"

RUN apt-get update \
    && apt-get install -y \
      cmake \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 4000

ENV RUST_MODE=production

WORKDIR /app

ARG GIT_HASH
ENV BUILD_ID=${GIT_HASH}

# Add all files
COPY ./ ./

# mount volumes
VOLUME [ "/app/config" ]

# Build and clean up
RUN cargo build

CMD ["cargo", "run"]
