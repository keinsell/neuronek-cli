default: check

target := arch()
operating_system := os()

alias fmt := format

# architectures := ["aarch64", "x86_64"]
# operating_systems := ["linux", "windows", "macos"]

# TODO: Cross-compilation
# TODO: Native compilation with attributes like [linux] (https://just.systems/man/en/attributes.html)

[private]
build-target target:
    # TODO: if current operating system is different than target use
    # cross-compilation toolkit or find a way to include cross-compilation
    # toolkit by default
    cargo zigbuild --bin=neuronek --target={{target}} --release

build-darwin-x64:
    cargo zigbuild --bin=neuronek --target="x86_64-apple-darwin" --release
#    docker run --rm -it -v $(pwd):/io -w /io messense/cargo-zigbuild cargo zigbuild --release --target x86_64-apple-darwin

build-windows-x64:
    cargo xwin build --bin=neuronek --target="x86_64-pc-windows-msvc" --release

build-linux-amd64:
    cargo zigbuild --bin=neuronek --target="x86_64-unknown-linux-musl" --release
    just package-makeself x86_64-unknown-linux-musl

build-docker:
    @echo "Add Dockerfile which will run tool and will operate on Unikernel or Distroless"
    
# Prepare release binaries for all platforms
@release: (build-target "x86_64-unknown-linux-gnu") (package-sea "x86_64-unknown-linux-gnu") (build-target "x86_64-pc-windows-gnu") (build-target "aarch64-unknown-linux-musl")
@package: (package-sea "x86_64-unknown-linux-gnu")

# Create a neuronek.run self-extracting archive which will install
# oneself inside home directory of linux operating system, this is
# potentially the easiest distribution method when it comes to 
# CLI applications without usage of package-manager.
#
# Supported operating systems: Linux, Darwin?
[linux]
@package-sea target="x86_64-unknown-linux-musl":
    cargo zigbuild --bin=neuronek --target={{target}} --release
    mkdir -p /tmp/npack && \
    mkdir -p dist && \
    cp target/{{target}}/release/neuronek /tmp/npack && \
    cp scripts/local_installation /tmp/npack/install && \
    chmod +x /tmp/npack/install && \
    makeself -q --tar-quietly /tmp/npack dist/neuronek.run "Neuronek CLI" ./install

# Create Debian archive
[linux]
@package-deb target="x86_64-unknown-linux-gnu":
    cargo bundle --release --target {{target}}
    cp target/{{target}}/release/bundle/deb/neuronek_*.deb dist/neuronek.deb

[linux]
@package-tarball target="x86_64-unknown-linux-musl":
    cargo dist build


package-appimage target:
    @echo TODO

package-flatpak target:
    @echo TODO

package-msi target="x86_64-pc-windows-gnu":
    vpk pack -u xyz.neuronek.cli -v 0.0.1-dev.0 -p /target/{{target}}/release -e neuronek.exe
#    cargo wix -p neuronek -t {{target}} --nocapture --no-build

package-dmg target:
    @echo TODO

format:
    cargo fmt

install:
    @echo "TODO";  

check:
    cargo check

lint:
    cargo clippy --fix
