{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: {
    devShells.default = import ./shell.nix { inherit pkgs; };
  };
}
