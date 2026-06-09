# Build stage
FROM rust:1.83-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM alpine:3.21
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/rustle /usr/local/bin/rustle

ENV LISTEN_PORT=8080
ENV TARGET_ADDR=127.0.0.1:8081

EXPOSE ${LISTEN_PORT}

ENTRYPOINT ["rustle"]
