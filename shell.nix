let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      gpgme gnupg openssl pkgconfig
    ];
  }
