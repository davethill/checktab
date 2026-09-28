{
  description = "checktab — small floating multi-tab checklist for niri";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];

      forSystem = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          checktab = pkgs.rustBundle.buildRustPackage {
            pname = "checktab";
            version = "0.1.0";
            src = self;

            nativeBuildInputs = [ pkgs.rustPlatform.rustLib ];
          };
        in
          {
            checktab = checktab;
            default = checktab;
          };

      devShell = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
          {
            default = pkgs.mkShell {
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
        packages = nixpkgs.lib.genAttrs systems forSystem;
        devShells = nixpkgs.lib.genAttrs systems devShell;
      };
}
