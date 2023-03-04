{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.default = pkgs.clangStdenv.mkDerivation {
      name = "remu";
      src = ./.;
      makeFlags = [ "PREFIX=$(out)" ];
      nativeBuildInputs = with pkgs; [ llvm ];
    };
    devShells.default = pkgs.mkShell.override {
      stdenv = pkgs.clangStdenv;
    } {
      buildInputs = with pkgs; [ musl ];
      nativeBuildInputs = with pkgs; [ llvm gdb ];
    };
  });
}
