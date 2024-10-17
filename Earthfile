VERSION --global-cache 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust


install:
  FROM rustlang/rust:nightly
  RUN rustup component add clippy rustfmt
  DO rust+INIT --keep_fingerprints=true
  DO rust+SET_CACHE_MOUNTS_ENV
  DO rust+CARGO --args="install cargo-binstall"

source:
    FROM +install
    WORKDIR /tmp
    COPY --keep-ts --dir src packages Cargo.lock Cargo.toml .
  
build:
    FROM +source
    COPY --keep-ts --dir src packages Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --bin neuronek" --output="release/[^/\.]+"
    SAVE ARTIFACT target/release/neuronek neuronek

# lint:
#   FROM +source
#   DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

# fmt:
#   FROM +lint
#   DO rust+CARGO --args="fmt --check"

all:
    BUILD +build
    # BUILD +fmt

docker:
    FROM registry.suse.com/bci/bci-micro:15.5
    COPY +build/neuronek neuronek
    ENTRYPOINT ["./neuronek"]
    SAVE IMAGE --push neuronek/cli:dev
