with import <nixpkgs> {};
pkgs.powerline-rs.overrideAttrs(old: {
  buildInputs = lib.remove pkgs.rustc old.buildInputs;
})
