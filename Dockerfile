# Build and Test
FROM rust:1.73.0 as builder
WORKDIR /usr/src/mycokv
COPY . .
RUN cargo test
RUN cargo build --release

# Final Image
FROM debian:12
COPY --from=builder /usr/src/mycokv/target/release/mycokv /usr/local/bin/mycokv
EXPOSE 6922
CMD ["mycokv"]