{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      fenix,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])

            xorg.libxcb
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
            wayland
            libxkbcommon
            pkg-config

            python3
            libGL
            libGLU
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="/run/opengl-driver/lib/:${
              pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  libGL
                  libGLU
                  wayland
                  libxkbcommon
                ]
              )
            }"
          '';
        };
      }
    );
}
