FROM rust:trixie AS builder
WORKDIR /app
COPY . ./
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM rust:trixie
COPY --from=builder /app/target/release/agforest-backend /app
CMD ["/app"]
