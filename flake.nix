{
  description = "The ggorg.xyz website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # The version of wasm-bindgen-cli needs to match the version in Cargo.lock
    nixpkgs-for-wasm-bindgen.url = "github:NixOS/nixpkgs/d0797a04b81caeae77bcff10a9dde78bc17f5661";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-root.url = "github:srid/flake-root";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.flake-root.flakeModule
        inputs.pre-commit-hooks-nix.flakeModule
      ];

      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', system, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          inherit (pkgs) lib;

          rustToolchainFor = p: p.rust-bin.nightly.latest.default.override {
            # Set the build targets supported by the toolchain,
            # wasm32-unknown-unknown is required for trunk
            targets = [ "wasm32-unknown-unknown" ];
          };

          craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain rustToolchainFor).overrideScope (_final: _prev: {
            # The version of wasm-bindgen-cli needs to match the version in Cargo.lock
            inherit (import inputs.nixpkgs-for-wasm-bindgen { inherit system; }) wasm-bindgen-cli;
          });

          # When filtering sources, we want to allow assets other than .rs files
          src = lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source
            filter = path: type:
              (lib.hasSuffix "\.html" path) ||
              (lib.hasSuffix "\.scss" path) ||
              # Example of a folder for images, icons, etc
              (lib.hasInfix "/assets/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type)
            ;
          };

          _module.args = { inherit pkgs; };

          # Common arguments can be set here to avoid repeating them later
          commonArgs = {
            inherit src;
            strictDeps = true;
            # We must force the target, otherwise cargo will attempt to use your native target
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

            buildInputs = [
              # Add additional build inputs here
            ];
          };

          # Build *just* the cargo dependencies, so we can reuse
          # all of that work (e.g. via cachix) when running in CI
          cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
            # You cannot run cargo test on a wasm build
            doCheck = false;
          });

          # Build the actual crate itself, reusing the dependency
          # artifacts from above.
          # This derivation is a directory you can put on a webserver.
          my-app = craneLib.buildTrunkPackage (commonArgs // {
            inherit cargoArtifacts;
            # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
            wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
              version = "0.2.95";
              hash = "sha256-prMIreQeAcbJ8/g3+pMp1Wp9H5u+xLqxRxL+34hICss=";
              cargoHash = "sha256-6iMebkD7FQvixlmghGGIvpdGwFNLfnUcFke/Rg8nPK4=";
            };
          });

        in
        {
          checks = {
            # Build the crate as part of `nix flake check` for convenience
            inherit my-app;

            # Run clippy (and deny all warnings) on the crate source,
            # again, reusing the dependency artifacts from above.
            #
            # Note that this is done as a separate derivation so that
            # we can block the CI if there are issues here, but not
            # prevent downstream consumers from building our crate by itself.
            rust-clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

            rust-fmt = craneLib.cargoFmt {
              inherit src;
            };
          };

          packages.default = my-app;

          apps.default = inputs.flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "serve-app" ''
              ${lib.getExe pkgs.darkhttpd} ${my-app}
            '';
          };

          devShells.default = craneLib.devShell {
            # Inherit inputs from checks.
            inherit (self') checks;

            shellHook = config.pre-commit.installationScript;

            packages = [
              pkgs.trunk
              pkgs.statix
              pkgs.docker
              config.treefmt.build.wrapper
            ] ++ (lib.attrValues config.treefmt.build.programs);
          };

          treefmt.config = {
            inherit (config.flake-root) projectRootFile;
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              prettier.enable = true;
            };
          };

          formatter = config.treefmt.build.wrapper;

          pre-commit.settings.hooks = {
            treefmt = {
              enable = true;
              packageOverrides.treefmt = config.treefmt.build.wrapper;
            };
            statix.enable = true;
          };
        };
    };
}
