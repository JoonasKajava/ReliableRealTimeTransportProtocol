{ pkgs, ... }:

{

  # https://devenv.sh/packages/
  packages = with pkgs; [ texlab texliveFull ];


  # https://devenv.sh/languages/
  languages.rust.enable = true;
  languages.rust.components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];


  enterShell = ''devenv info'';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
