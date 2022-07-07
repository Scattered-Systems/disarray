FROM jo3mccain/rusty

ADD .. /project
WORKDIR /project

COPY .. .
RUN cargo build --release --workspace && \
    cargo test --all-features