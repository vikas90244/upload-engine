FROM rust:1.84 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/upload-engine .

EXPOSE 8080

CMD ["./upload-engine"]