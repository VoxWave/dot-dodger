with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "env";
  nativeBuildInputs = [ 
    pkg-config
    # cmake
  ];
  buildInputs = [
    alsa-lib.dev
    systemd
    # fontconfig.dev
    # xorg.libX11
    # xorg.libX11.dev
    # xorg.libXcursor
    # xorg.libXrandr
    # xorg.libXi
    # vulkan-tools
    # vulkan-headers
    # vulkan-loader
    # vulkan-validation-layers
  ];
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXrandr
    # xorg.libXi
    # pkgs.vulkan-loader
  ]}"'';
}