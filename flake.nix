{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import "${fenix}/overlay.nix") ];
        pkgs = import nixpkgs { inherit system overlays; };

        rust-toolchain = pkgs.fenix.stable.withComponents [
          "cargo"
          "clippy"
          "rust-analyzer"
          "rust-src"
          "rustc"
          "rustfmt"
        ];

        llvm = pkgs.llvmPackages_19;
      in
      {
        devShells.default =
          with pkgs;
          mkShell.override { stdenv = llvm.libcxxStdenv; } {
            nativeBuildInputs = [
              # rust
              rust-toolchain

              # c++
              cmake
              ninja
              (llvm.clang-tools.override { enableLibcxx = true; })
            ];
          };
      }
    );
}
