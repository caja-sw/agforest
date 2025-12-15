FROM rust:trixie AS builder
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM rust:trixie
COPY --from=builder /app/target/release/agforest-backend /app
CMD ["/app"]
