FROM rust:trixie AS builder
COPY . /app
WORKDIR /app
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM rust:trixie
COPY --from=builder /app/target/release/agforest-backend /app
CMD ["/app"]
