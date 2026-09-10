{
  description = "signal-persona - ordinary Persona lifecycle signal contract.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-src"
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        ethosFilter = path: type: type == "regular" && pkgs.lib.hasSuffix ".ethos" path;
        examplesFilter = path: _type: builtins.match ".*/examples(/.*)?$" path != null;
        sourceFilter = path: type:
          type == "directory"
          || (craneLib.filterCargoSources path type)
          || (ethosFilter path type)
          || (examplesFilter path type);
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = sourceFilter;
          name = "source";
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        checks = {
          build = craneLib.cargoBuild (commonArgs // { inherit cargoArtifacts; });
          test = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          test-generated-contract = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; cargoTestExtraArgs = "--test generated_contract"; });
          test-datom = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; cargoTestExtraArgs = "--features datom --test generated_contract"; });
          test-doc = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; cargoTestExtraArgs = "--doc"; });
          doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; RUSTDOCFLAGS = "-D warnings"; });
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (commonArgs // { inherit cargoArtifacts; cargoClippyExtraArgs = "--all-targets --all-features -- -D warnings"; });
        };
        devShells.default = pkgs.mkShell {
          name = "signal-persona";
          packages = [
            pkgs.jujutsu
            pkgs.pkg-config
            toolchain
          ];
        };

        formatter = pkgs.nixfmt;
      }
    );
}
