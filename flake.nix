{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };
  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = {
        config,
        self',
        pkgs,
        lib,
        system,
        ...
      }: let
        fenixPkgs = inputs.fenix.packages.${system};
        toolchain = fenixPkgs.combine [
          # Default Rust tools
          fenixPkgs.stable.cargo
          fenixPkgs.stable.clippy
          fenixPkgs.stable.rust-src
          fenixPkgs.stable.rustc
          fenixPkgs.stable.rustfmt
        ];
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        buildInputs = with pkgs; [];
        nativeBuildInputs = with pkgs; [];
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit (cargoToml.package) name version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          RUST_BACKTRACE = "full";

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;
        };

        # Rust dev environment
        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

          packages =
            nativeBuildInputs
            ++ buildInputs
            ++ [toolchain]
            ++ [
              # Nix tools
              pkgs.nil
              pkgs.alejandra

              pkgs.linuxKernel.packages.linux_zen.perf
            ];
        };

        # Add your auto-formatters here.
        # cf. https://numtide.github.io/treefmt/
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };
      };
    };
}