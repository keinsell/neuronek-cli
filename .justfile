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

# Prepare release binaries for all platforms
@release: (build-target "x86_64-unknown-linux-gnu") (package-makeself "x86_64-unknown-linux-gnu") (build-target "x86_64-pc-windows-gnu") (build-target "aarch64-unknown-linux-musl")
@package: (package-makeself "x86_64-unknown-linux-gnu")

# Create a neuronek.run self-extracting archive which will install
# oneself inside home directory of linux operating system, this is
# potentially the easiest distribution method when it comes to 
# CLI applications without usage of package-manager.
#
# Supported operating systems: Linux, Darwin?
[private]
@package-makeself target:
    mkdir -p /tmp/npack && \
    mkdir -p dist && \
    cp target/{{target}}/release/neuronek /tmp/npack && \
    cp scripts/local_installation /tmp/npack/install && \
    chmod +x /tmp/npack/install && \
    makeself -q --tar-quietly /tmp/npack dist/neuronek.run "Neuronek CLI" ./install

package-appimage target:
    @echo TODO

package-flatpak target:
    @echo TODO

package-msi target:
    @echo TODO

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
