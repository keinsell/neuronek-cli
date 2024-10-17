VERSION 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust

FROM rustlang/rust:nightly

# Install dependencies that are used across jobs
DO rust+INIT --keep_fingerprints=true
DO rust+SET_CACHE_MOUNTS_ENV
DO rust+CARGO --args="install cargo-binstall"

WORKDIR /tmp

build-all:
  BUILD --platform=linux/amd64 +build
  
build:
    COPY --keep-ts --dir src packages Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --bin neuronek" --output="release/[^/\.]+"
    SAVE ARTIFACT target/release/neuronek neuronek

docker:
    FROM registry.suse.com/bci/bci-micro:15.5
    COPY +build/neuronek neuronek
    ENTRYPOINT ["./neuronek"]
    SAVE IMAGE --push neuronek/cli:dev
