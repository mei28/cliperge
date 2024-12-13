{
  description = "A CLI tool to combine and copy file contents to the clipboard";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs =
    { self, nixpkgs }:
    {
      packages = {
        # Linux
        x86_64-linux = self.defaultPackage.x86_64-linux;

        # macOS
        x86_64-darwin = self.defaultPackage.x86_64-darwin;
      };

      defaultPackage = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        pkgs.stdenv.mkDerivation {
          pname = "cliperge";
          version = "0.3.0";

          src = self;

          nativeBuildInputs = [
            pkgs.rustc
            pkgs.cargo
          ];

          buildPhase = ''
            cargo build --release
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/release/cliperge $out/bin/
          '';

          meta = with pkgs.lib; {
            description = "A CLI tool to combine and copy file contents to the clipboard";
            license = licenses.mit;
            maintainers = [ maintainers.mei28 ];
          };
        }
      );
    };
}
