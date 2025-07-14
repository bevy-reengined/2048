{
  description = "Rust dev shell template, used for bevy projects";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
      in
      {
        devShells.default =
          with pkgs;
          mkShell rec {
            packages = [
              toolchain
            ];
            buildInputs = [
              vulkan-loader
              wayland
              alsa-lib
              udev
              libxkbcommon
            ];
            nativeBuildInputs = [
            ];
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      }
    );
}
