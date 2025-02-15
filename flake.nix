{
  description = "A CLI tool to combine the contents of multiple files and copy to the clipboard";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs?ref=nixpkgs-unstable";

  outputs = { self, nixpkgs }:
  let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
  in
  {
    packages =
      builtins.foldl'
      (acc: system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        acc // {
          ${system} = pkgs.rustPlatform.buildRustPackage {
            pname = "cliperge";
            version = "0.3.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        }
      ) {} systems;

    defaultPackage = {
      x86_64-linux = self.packages.x86_64-linux;
      aarch64-linux = self.packages.aarch64-linux;
      x86_64-darwin = self.packages.x86_64-darwin;
      aarch64-darwin = self.packages.aarch64-darwin;
    };

    defaultApp = {
      x86_64-linux = {
        type = "app";
        program = "${self.packages.x86_64-linux}/bin/cliperge";
      };
      aarch64-linux = {
        type = "app";
        program = "${self.packages.aarch64-linux}/bin/cliperge";
      };
      x86_64-darwin = {
        type = "app";
        program = "${self.packages.x86_64-darwin}/bin/cliperge";
      };
      aarch64-darwin = {
        type = "app";
        program = "${self.packages.aarch64-darwin}/bin/cliperge";
      };
    };
  }
}
