FROM node:22 AS build-js

WORKDIR /usr/src/myivo

COPY frontend/package*.json .
RUN npm install

COPY frontend .

# tailwind classes are in the backend's HTML template files
COPY server/templates templates
RUN sed -i "s|../../../server/templates|../../templates|" src/css/tailwind.css

RUN npm run build:production

FROM rust:1.85 AS builder-rs

WORKDIR /usr/src/myivo-server
COPY server .

# point to minimised, production versions of build artefacts
RUN sed -i "s|build/app|build/app.min|g" templates/index.html
RUN cargo install --profile release --locked --path .

# run on different image
FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y openssl ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /root

COPY --from=build-js /usr/src/myivo/build ./build
COPY --from=builder-rs /usr/local/cargo/bin/myivo-server /usr/local/bin/

EXPOSE 8080

CMD ["myivo-server"]
