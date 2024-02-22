{ pkgs ? import <nixpkgs> { }, lib }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "sun_server";
  version = "1.0.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgs.lib.cleanSource ../.;
  buildInputs = with pkgs; [
    surrealdb
    clang
   ];
#  doCheck = false;
  LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
}
