FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /src
COPY . .

RUN cargo build --release

FROM alpine:3.19
WORKDIR /app
COPY --from=builder /src/target/release/logscan /usr/local/bin/logscan

ENTRYPOINT ["logscan"]
CMD ["--help"]
