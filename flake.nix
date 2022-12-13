# based on: https://github.com/cargo2nix/cargo2nix/blob/c149357cc3d17f2849c73eb7a09d07a307cdcfe8/examples/1-hello-world/flake.nix
{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;

    flake-utils.lib.eachDefaultSystem (system:
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.61.0";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "rustfmt" "clippy" ];
        };

      in
      rec {
        packages = {
          aoc2022rust = (rustPkgs.workspace.aoc2022rust { }).bin;
          default = packages.aoc2022rust;
        };

        devShell = rustPkgs.workspaceShell {
          # packages = [ ];
        };

      }
    );
}
