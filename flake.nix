{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system} = {
        bad-apple-frames = pkgs.callPackage ./frames.nix { };
        badge-apple = pkgs.callPackage ./package.nix { };
        default = self.packages.${system}.badge-apple;
      };
      devShells.${system}.default = pkgs.callPackage ./shell.nix { };
    };
}
