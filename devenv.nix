{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env = {
    PROJECT = "neuronek-cli";
    RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  };

  dotenv = {
    enable = true;
    disableHint = true;
  };

  devenv = {
    debug = false;
    warnOnNewVersion = true;
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    openssl
    onefetch
    direnv
    nix-direnv
    nix-direnv-flakes
    sea-orm-cli
    sccache
  ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      rustflags = "-Z threads=32";
      components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
    };
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.motd.exec = "onefetch";
  scripts.build.exec = "cargo build";
  scripts.test.exec = "cargo test";

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
