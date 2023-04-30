{ pkgs, openssl, ... }:

{
  packages = [
    pkgs.jq
    pkgs.git
    pkgs.toilet
    pkgs.openssl
  ];

  scripts.hello.exec = "toilet -f mono12 -F metal 'Welcome'";

  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustversion
    version = "stable";
  };

  scripts.tests.exec = ''
    cargo test 
  '';

  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
    shellcheck.enable = true;
    mdsh.enable = true;
  };

  enterShell = ''
    hello
  '';

}
