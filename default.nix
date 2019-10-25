{ pkgs ? import <nixpkgs> {} }:

(pkgs.callPackage ./Cargo.nix {}).workspaceMembers.powerline-rs.build
