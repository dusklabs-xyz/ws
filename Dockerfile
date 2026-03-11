FROM rust:1.82-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/ws-echo /usr/local/bin/ws-echo
EXPOSE 8080
CMD ["ws-echo"]
