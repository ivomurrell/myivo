FROM rust:1.61 as builder

WORKDIR /usr/src/myivo-server
COPY server .

RUN cargo install --path .

# run on different image
FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

COPY frontend .
COPY --from=builder /usr/local/cargo/bin/myivo-server /usr/local/bin/myivo-server

EXPOSE 8010

CMD ["myivo-server"]