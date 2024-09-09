{ pkgs
, lib
, config
, inputs
, ...
}:
{
  # https://devenv.sh/basics/
  env = {
    PROJECT = "neuronek-cli";
    RUSTC_WRAPPER = "sccache";
    RUST_BACKTRACE = "full";
    CARGO_LOG = "warn";
    SCCACHE_LOG = "warn";
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
    rustfilt
    sqlite
  ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      rustflags = "-Z threads=0";
      mold.enable = true;
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-std" "rust-src" "llvm-tools" "rust-docs" ];
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
