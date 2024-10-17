VERSION 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust

FROM rust:slim-bookworm
WORKDIR /rustexample

# build creates the binary target/release/example-rust
build:
    # CARGO function adds caching to cargo runs.
    # See https://github.com/earthly/lib/tree/main/rust
    DO rust+INIT --keep_fingerprints=true
    COPY --keep-ts --dir src Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --bin neuronek" --output="release/[^/\.]+"
    SAVE ARTIFACT target/release/neuronek neuronek

# docker creates docker image earthly/examples:rust
docker:
    FROM debian:bookworm-slim
    COPY +build/neuronek neuronek
    EXPOSE 9091
    ENTRYPOINT ["./neuronek"]
    SAVE IMAGE --push neuronek/cli:dev
