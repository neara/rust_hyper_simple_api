FROM rust:1.43.0-slim-buster AS build

LABEL maintainer="Chiliseed LTD"

ENV RUST_LOG=INFO

RUN apt-get update && apt-get upgrade -y

WORKDIR /app

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

RUN cargo install --path .

FROM debian:buster-slim AS prod

ENV RUST_LOG=INFO

WORKDIR /app

COPY --from=build /usr/local/cargo/bin/demo_backend /app/

EXPOSE 7878/tcp

CMD ["/app/demo_backend"]
