{ pkgs
, lib
, config
, inputs
, ...
}:
  let libPath = with pkgs; lib.makeLibraryPath [
    sqlite
    openssl
  ];
  in
{
  # https://devenv.sh/basics/
  env = {
    PROJECT = "neuronek-cli";
    # RUSTC_WRAPPER = "sccache";
    RUST_BACKTRACE = "full";
    # LIBCLANG_PATH= pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
    # CARGO_LOG = "warn";
    # SCCACHE_LOG = "warn";
    # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    # LD_LIBRARY_PATH = libPath;
    # BINDGEN_EXTRA_CLANG_ARGS =
    # # Includes with normal include path
    # (builtins.map (a: ''-I"${a}/include"'') [
    #   pkgs.libvmi
    #   pkgs.glibc.dev
    # ])
    # # Includes with special directory paths
    # ++ [
    #   ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
    #   ''-I"${pkgs.glib.dev}/include/glib-2.0"''
    #   ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    # ];
  };

  dotenv = {
    enable = true;
    disableHint = true;
  };

  devenv = {
    debug = false;
    warnOnNewVersion = false;
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    rustup
    openssl
    onefetch
    direnv
    nix-direnv
    nix-direnv-flakes
    sccache
    adrgen
    cargo-temp
    cargo-chef
    cargo-vet
    cargo-make
    cargo-cross
    cargo-binstall
    cargo-bundle
    cargo-cranky
    cargo-msrv
    cargo-zigbuild
    cargo-nextest
    cargo-dist
    cargo-xwin
    cargo-xbuild
    cargo-bundle
    cargo-deb
    cargo-expand    
    earthly
    cargo-autoinherit
    # pkg-config
    rustfilt
    sqlite
    llvmPackages.bintools
          llvmPackages_latest.llvm
      llvmPackages_latest.bintools
      zlib.out
      rustup
      xorriso
      grub2
      qemu
      llvmPackages_latest.lld
      python3
      makeself
      upx
      candle
      dotnet-sdk
      dotnet-runtime              
      ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      mold.enable = true;
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-std" "rust-src" "llvm-tools" "rust-docs" "llvm-tools-preview" ];
      targets = [
      "x86_64-unknown-linux-gnu"
      "x86_64-unknown-linux-musl"
      "x86_64-pc-windows-gnu"
      "x86_64-pc-windows-msvc"
      "x86_64-apple-darwin"
      "aarch64-apple-darwin"
      "aarch64-unknown-linux-musl"
      "aarch64-pc-windows-msvc"
      ];
    };
    zig = {
      enable = true;
    };
    dotnet = {
      enable = true;
    };
  };

  # https://devenv.sh/processes/
   processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.motd.exec = "onefetch";
  scripts.lint.exec = ''
   cargo clippy --fix
   cargo fmt --all
  '';
  scripts.build.exec = "cargo build";
  scripts.test.exec = "cargo test";
  scripts.dev.exec = "bacon";
  scripts.nd.exec = "cargo run";

  enterShell = ''
    motd
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
