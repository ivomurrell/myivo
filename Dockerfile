FROM node:16 as build-js

WORKDIR /usr/src/myivo

COPY frontend/package*.json .
RUN npm install

COPY frontend .
RUN npm run build:production

FROM rust:1.74-buster as builder-rs

WORKDIR /usr/src/myivo-server
COPY server .

RUN cargo install --profile release --locked --path .

# run on different image
FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

WORKDIR /root

COPY --from=build-js /usr/src/myivo/index.html ./
# point to minimised, production versions of build artefacts
RUN sed -i "s|build/app|build/app.min|g" index.html
COPY --from=build-js /usr/src/myivo/images ./images 
COPY --from=build-js /usr/src/myivo/fonts ./fonts
COPY --from=build-js /usr/src/myivo/build ./build
COPY --from=builder-rs /usr/local/cargo/bin/myivo-server /usr/local/bin/

EXPOSE 8080

CMD ["myivo-server"]
