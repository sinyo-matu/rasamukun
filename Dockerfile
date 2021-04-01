FROM rust as builder
WORKDIR /usr/src/rasamukun
COPY . .
RUN cargo install --path . 

FROM debian:buster-slim
RUN apt-get update&&apt-get install -y openssl libssl-dev curl
COPY --from=builder /usr/local/cargo/bin/rasamukun /usr/local/bin/rasamukun
CMD ["rasamukun"]