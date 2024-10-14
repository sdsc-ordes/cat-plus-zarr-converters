{
  description = "tripsu";

  nixConfig = {
    extra-substituters = [
      # Nix community's cache server
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    # Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # You can access packages and modules from different nixpkgs revs
    # at the same time. Here's an working example:
    nixpkgsStable.url = "github:nixos/nixpkgs/nixos-23.11";
    # Also see the 'stable-packages' overlay at 'overlays/default.nix'.

    flake-utils.url = "github:numtide/flake-utils";

    # The Rust overlay to include the latest toolchain.
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    let
      # This is string (without toString it would be a `path` which is put into the store)
      rootDir = toString ./. + "../../..";
    in
    flake-utils.lib.eachDefaultSystem
      # Creates an attribute map `{ devShells.<system>.default = ...}`
      # by calling this function:
      (
        system:
        let
          overlays = [ (import rust-overlay) ];

          # Import nixpkgs and load it into pkgs.
          # Overlay the rust toolchain
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          # Set the rust toolchain from the `rust-toolchain.toml`.
          rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ../../rust-toolchain.toml;

          # Things needed only at compile-time.
          packagesBasic = with pkgs; [
            findutils
            coreutils
            bash
            zsh
            curl
            git
            jq
          ];

          # Things needed only at compile-time.
          packagesDev = with pkgs; [
            rustToolchain
            cargo-watch
            just

            skopeo
            dasel
          ];
        in
        with pkgs;
        rec {
          devShells = {
            default = mkShell {
              packages = packagesBasic ++ packagesDev;
            };

            ci = mkShell {
              packages = packagesBasic ++ packagesDev;

              # Due to some weird handling of TMPDIR inside containers:
              # https://github.com/NixOS/nix/issues/8355
              # We have to reset the TMPDIR to make `nix build` work inside
              # a development shell.
              # Without `nix develop` it works.
              shellHook = "unset TMPDIR";
            };
          };
        }
      );
}
