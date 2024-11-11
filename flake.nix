{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # rust stuff
            cargo
            cargo-leptos
            cargo-generate
            rust-analyzer
            leptosfmt
            stylance-cli

            # npm stuff
            nodePackages.pnpm
            nodejs_22
            sass

            # other language servers
            tailwindcss-language-server
            emmet-language-server

            # utils
            binaryen
            openssl
            pkg-config
            eza
            fd
            (rust-bin.selectLatestNightlyWith (
              toolchain:
              toolchain.default.override {
                extensions = [ "rust-src" ];
                targets = [ "wasm32-unknown-unknown" ];
              }
            ))
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}
