FROM rust as builder
WORKDIR /usr/src/rasamukun
COPY . .
RUN cargo install --path . 

FROM ubuntu
RUN apt-get update&&apt-get install -y openssl libssl-dev
COPY --from=builder /usr/local/cargo/bin/rasamukun /usr/local/bin/rasamukun
CMD ["rasamukun"]