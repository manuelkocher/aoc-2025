{ pkgs, lib, config, ... }:
{
  languages.rust = {
    # enable rust for this repo
    enable = true;
    # install compiler, pkg manager, linter and formater
    components = [ "rustc" "cargo" "clippy" "rustfmt" ];
  };

  # enable git hooks for formater and linter
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}