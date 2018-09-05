with import <nixpkgs> {};
with rustPlatform;

let
  #app = buildRustPackage rec {
  #  name = "smart-meter-server-${version}";
  #  version = "0.1.0";

  #  src = ./.;
  #  cargoSha256 = "0qwwjz98fyfirdvwn3v12s70wbyvmgx202ykk6yjzm2b6jkpr3rr";
  #};

  # XXX: YUCK
  app = pkgs.writeShellScriptBin "smart-meter-server" ''
    ${./target/debug/smart-meter-server}
  '';
in
  app
