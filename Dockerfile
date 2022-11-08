FROM scsys/rust:debian-nightly as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN rustup default nightly

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --workspace

FROM builder as publisher

FROM builder as testing

RUN cargo test --all --verbose