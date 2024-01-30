FROM rust:1.75
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /app
RUN apt update && apt install ca-certificates openssl
COPY --from=0 /app/target/release/sms-gateway-rust .
RUN chmod +x sms-gateway-rust
ENV ROCKET_ADDRESS 0.0.0.0
CMD ./sms-gateway-rust
