# Build
FROM rust:alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Expose
FROM alpine:latest
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/raccoon-api .

USER 1000:1000

EXPOSE 3000

CMD ["./raccoon-api"]