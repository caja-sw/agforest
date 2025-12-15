FROM rust:trixie AS builder
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=builder /app/target/release/agforest-backend /app
CMD ["/app"]
