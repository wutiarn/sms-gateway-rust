FROM rust:1.58.0
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:21.10
WORKDIR /app
COPY --from=0 /app/target/release/sms-gateway-rust .
RUN chmod +x sms-gateway-rust
CMD ./sms-gateway-rust
