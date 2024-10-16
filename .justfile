default: check

target := arch()
operating_system := os()

alias lint := check
alias fmt := format

# architectures := ["aarch64", "x86_64"]
# operating_systems := ["linux", "windows", "macos"]

# TODO: Cross-compilation
# TODO: Native compilation with attributes like [linux] (https://just.systems/man/en/attributes.html)
# TODO: 

[private]
release-target target:
    @echo operating_system
    sudo cross build --bin=neuronek --target={{target}} --release

# Prepare release binaries for all platforms
@release: (release-target "aarch64-unknown-linux-gnu") (release-target "x86_64-pc-windows-gnu")

package:
    @echo "TODO";

format:
    cargo fmt

install:
    @echo "TODO";  

check:
    cargo check

lint:
    cargo clippy
