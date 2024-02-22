{ pkgs ? import <nixpkgs> { } }:
let
  runtimeLibs = [
      pkgs.libclang
      pkgs.rocksdb
  ];
in
  pkgs.mkShell {
    # Get dependencies from the main package
    inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
    # Additional tooling
    buildInputs = with pkgs; [
      cargo
      cargo-watch
      rustc
      rustup
      clippy
      rust-analyzer
      pkg-config
      bacon
      nodePackages_latest.pnpm
      surrealdb
      libclang
    ];
    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath runtimeLibs}";
  }
