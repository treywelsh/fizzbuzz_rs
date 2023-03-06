FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin fizzbuzz

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/fizzbuzz /usr/local/bin/
EXPOSE 8080:8080
ENTRYPOINT ["/usr/local/bin/fizzbuzz"]