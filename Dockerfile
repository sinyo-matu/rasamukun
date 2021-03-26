FROM rust as builder
WORKDIR /usr/src/rasamukun
COPY . .
RUN cargo install --path . 

FROM ubuntu
RUN apt
COPY --from=builder /usr/local/cargo/bin/rasamukun /usr/local/bin/rasamukun
CMD ["rasamukun"]