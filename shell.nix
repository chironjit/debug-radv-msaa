{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.libxkbcommon
    pkgs.xorg.libxcb
    pkgs.wayland
    pkgs.vulkan-loader
  ];

  env = {
    NIX_LDFLAGS = "-rpath ${
      pkgs.lib.makeLibraryPath [
        pkgs.wayland
        pkgs.vulkan-loader
      ]
    }";
  };
}

