{
  description = "checktab — small floating multi-tab checklist for niri";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];

      forSystem = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          checktab = pkgs.rustPlatform.buildRustPackage {
            pname = "checktab";
            version = "0.1.0";
            src = self;

            cargoLock = { lockFile = ./Cargo.lock; };

            nativeBuildInputs = [ pkgs.makeWrapper ];

            postInstall = ''
              wrapProgram $out/bin/checktab \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [
                  pkgs.wayland
                  pkgs.libxkbcommon
                  pkgs.libGL
                  pkgs.vulkan-loader
                  pkgs.libx11
                  pkgs.libxcursor
                  pkgs.libxi
                  pkgs.libxrandr
                ]}
            '';
          };
        in
          {
            checktab = checktab;
            default = checktab;
          };

      devShell = system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          runtimeLibs = [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.libGL
            pkgs.vulkan-loader
            pkgs.libx11
            pkgs.libxcursor
            pkgs.libxi
            pkgs.libxrandr
          ];
        in
          {
            default = pkgs.mkShell {
              packages = [
                pkgs.rustc
                pkgs.cargo
                pkgs.clippy
                pkgs.jq
                pkgs.niri
              ] ++ runtimeLibs;

              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
            };
          };
    in
      {
        packages = nixpkgs.lib.genAttrs systems forSystem;
        devShells = nixpkgs.lib.genAttrs systems devShell;
        overlays.default = final: prev: {
          checktab = self.packages.${final.stdenv.hostPlatform.system}.default;
        };
      };
}
