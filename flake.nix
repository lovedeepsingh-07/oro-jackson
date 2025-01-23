{
  description = "a flake for oro-jackson dev environment";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/6c90912761c43e22b6fb000025ab96dd31c971ff";
    # deno_2_1_4-pkgs.url =
    #   "github:nixos/nixpkgs/4989a246d7a390a859852baddb1013f825435cee";
    node_22_10_0-pkgs.url =
      "github:nixos/nixpkgs/566e53c2ad750c84f6d31f9ccb9d00f823165550";
    rust_1_82_0-pkgs.url =
      "github:nixos/nixpkgs/566e53c2ad750c84f6d31f9ccb9d00f823165550";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # deno_pkgs = inputs.deno_2_1_4-pkgs.legacyPackages.${system};
        node_pkgs = inputs.node_22_10_0-pkgs.legacyPackages.${system};
        rust_pkgs = inputs.rust_1_82_0-pkgs.legacyPackages.${system};
      in {
        formatter =
          pkgs.nixfmt-classic; # formatter for .nix files, just run `nix fmt .` to format the entire directory
        devShell = pkgs.mkShell {
          packages = [
            pkgs.go-task
			# deno_pkgs.deno
			node_pkgs.nodejs_22
            rust_pkgs.rustc
            rust_pkgs.cargo
            rust_pkgs.rustfmt
            rust_pkgs.just
          ];
        };
      });
}
