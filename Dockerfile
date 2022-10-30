FROM rust:1.61

WORKDIR /usr/local/app/reach

COPY ./dummy.rs ./dummy.rs
COPY ./Cargo.toml ./Cargo.toml

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo build --release

ENTRYPOINT ["./target/release/reach"]