FROM docker.io/rust:1.66-slim-buster as builder

WORKDIR /application

RUN apt-get update && \
    apt-get install -y libssl-dev

ADD Cargo.toml /application/Cargo.toml
ADD server /application/server
ADD client /application/client
ADD shared /application/shared

RUN cargo test && \
    cargo build --release && \
    mkdir -p /application/bin && \
    mv /application/target/release/server /application/bin/server && \
    mv /application/target/release/pedreiro /application/bin/pedreiro

FROM docker.io/debian:buster-slim

ARG GIT_REFERENCE=HEAD
ARG VERSION

LABEL "org.opencontainers.image.source"="https://github.com/neurono-ml/pedreiro-api"
LABEL "org.opencontainers.image.revision"="${GIT_REFERENCE}"
LABEL "org.opencontainers.image.version"="${VERSION}"
LABEL "ml.neurono/healthz"="/health"

ENV RUST_LOG=info
ENV ACCEPT_INVALID_CERTIFICATES false
ENV HOST 0.0.0.0
ENV PORT 8080

WORKDIR /application

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/* && \
    update-ca-certificates

COPY --from=builder  /application/bin/* /usr/bin/

    
