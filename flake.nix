{
  description = "Music sheet generation dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = mkShell rec {
          buildInputs = [
            # Rust
            rust-bin.nightly."2025-02-17".minimal

            # misc. libraries
            pkg-config
            pkgs.alsa-lib
            alsa-firmware
            alsa-plugins
            alsa-utils
            sof-firmware

            # GUI libs
            libxkbcommon
            libGL
            fontconfig
            mesa
            libxkbcommon
            pipewire
            pulseaudio
            # pipewire-alsa

            pkgs.wireplumber

            # debugging
            glxinfo

            # wayland libraries
            wayland

            # x11 libraries
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11

            # GUI app runtime dependencies
            pkgs.lilypond
            pkgs.fluidsynth
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          ALSA_PLUGIN_DIR = "${pipewire}/lib/alsa-lib";
          ALSA_CONFIG_PATH = "${pipewire}/share/alsa/alsa.conf.d/99-pipewire-default.conf";
          PIPEWIRE_RUNTIME_DIR = "/run/user/1000"; # Use your actual user ID
          PULSE_SERVER = "unix:${PIPEWIRE_RUNTIME_DIR}/pipewire-0";
          XDG_RUNTIME_DIR = "/run/user/1000"; # Use your actual user ID
        };
      });
}
