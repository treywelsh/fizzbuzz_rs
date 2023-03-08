FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin fizzbuzz

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/fizzbuzz /usr/local/bin/
ENV RUST_LOG=debug
ENTRYPOINT ["/usr/local/bin/fizzbuzz"]