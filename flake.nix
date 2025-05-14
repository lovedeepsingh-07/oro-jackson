{
  description = "oro-jackson";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/ebe2788eafd539477f83775ef93c3c7e244421d3";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust_1_84_0-pkgs.url =
      "github:nixos/nixpkgs/d98abf5cf5914e5e4e9d57205e3af55ca90ffc1d";
    bun_1_2_0-pkgs.url =
      "github:nixos/nixpkgs/f898cbfddfab52593da301a397a17d0af801bbc";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, crane, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        bun-pkgs = inputs.bun_1_2_0-pkgs.legacyPackages.${system};
        rust-pkgs = inputs.rust_1_84_0-pkgs.legacyPackages.${system};
        rust = pkgs.rust-bin.stable."1.84.0".default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        build-deps = [ ];
        package = craneLib.buildPackage {
          src = ./.;
          strictDeps = true;
          buildInputs = build-deps;
        };
      in {
        formatter = pkgs.nixfmt-classic;
        devShell = pkgs.mkShell {
          packages = [ rust rust-pkgs.just rust-pkgs.cargo-watch bun-pkgs.bun ]
            ++ build-deps;
        };
        packages.default = package;
        apps.default = {
          type = "app";
          program = "${package}/bin/raesan_registry-bin";
        };
      });
}
