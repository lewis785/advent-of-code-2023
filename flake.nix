{
  description = "A simple flake for Rust and cowsay development";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # 1.74.0 release
    rust_dep.url = "github:NixOS/nixpkgs/fd04bea4cbf76f86f244b9e2549fca066db8ddff";
  };

  outputs = { self, flake-utils, rust_dep, nixpkgs }@input: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = input.nixpkgs.legacyPackages.${system};
        rust_dep = input.rust_dep.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rust_dep.rustc
            rust_dep.cargo
            pkgs.zsh
          ];

          shellHook = ''
            echo "Welcome to nix rust shell"
            exec zsh
          '';
        };
      }
    );
}