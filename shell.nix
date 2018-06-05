with import <nixpkgs> {};
pkgs.powerline-rs.overrideAttrs(old: {
  buildInputs = lib.filter (pkg: pkg != pkgs.rustc && pkg != pkgs.cargo) old.buildInputs;
})
