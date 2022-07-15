FROM jo3mccain/rusty as builder

ENV PORT=8080 \
    RUST_LOG=info

ADD . /project
WORKDIR /project

COPY . .

RUN cargo build --release --color always && \
    cargo test --all-features --color always && \
    cargo package --allow-dirty --quiet --color always


EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp