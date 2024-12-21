{
  description = "A Rust project managed with Nix flakes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";  # Fetch nixpkgs
    flake-utils.url = "github:numtide/flake-utils";  # Utility for managing multi-systems
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy       # Rust linter
            pkgs.rustfmt      # Rust formatter
          ];
        };

        # Optionally define a package for building
        # Default package is built with cargo.
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "my-rust-project";
          version = "1.0.0";

          src = ./.;

          buildInputs = [
            pkgs.rustc
            pkgs.cargo
          ];

          buildPhase = ''
            cargo build --release
          '';

          installPhase = ''
            install -D target/release/my-rust-project $out/bin/my-rust-project
          '';
        };
      });
}