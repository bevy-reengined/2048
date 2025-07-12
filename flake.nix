{
  description = "Rust dev shell template, used for bevy projects";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default =
          with pkgs;
          mkShell rec {
            packages = [
              rust-analyzer
              rustfmt
              clippy
              vulkan-tools
            ];
            buildInputs = [
              vulkan-loader
              wayland 
              alsa-lib
              udev
              libxkbcommon
            ];
            nativeBuildInputs = [
              rustc
              cargo
              pkg-config
              mold
            ];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      }
    );
}
