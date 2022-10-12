FROM jo3mccain/rusty:nightly as builder-base

RUN yum install -y \
    protobuf-c

FROM builder-base as builder 

ADD . /project
WORKDIR /project

COPY .. .

RUN cargo build --release && \
    cargo test --all-features -q --release

FROM builder as latest

ENV PORT=8080 \
    RUST_LOG=info

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp