{ pkgs, rust-pkgs, bun-pkgs, ctx }:

pkgs.mkShell {
  packages = [ ctx.rust rust-pkgs.just rust-pkgs.cargo-watch bun-pkgs.bun ]
    ++ ctx.build-deps;
}
