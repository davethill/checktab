{
  description = "checktab — small floating multi-tab checklist for niri";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      forSystem = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
          {
            packages.checktab = pkgs.rustBundle.buildRustPackage {
              pname = "checktab";
              version = "0.1.0";
              src = self;

              nativeBuildInputs = [ pkgs.rustPlatform.rustLib ];
            };

            devShells.default = pkgs.mkShell {
              packages = [
                pkgs.rustc
                pkgs.cargo
                pkgs.clippy
                pkgs.jq
                pkgs.niri
              ];
            };
          };
    in
      {
        x86_64-linux = forSystem "x86_64-linux";
        aarch64-linux = forSystem "aarch64-linux";
      };
}
