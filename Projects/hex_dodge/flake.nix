{
  description = "hex-dodge — a tiny arcade game built on the hex engine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      # The game builds with cargo against the hex engine at ../hex (see the
      # README). This shell gives you everything needed to build and run it on
      # NixOS — shaderc is required because the engine's vulkano-shaders
      # compiles GLSL at build time.
      #
      #   nix develop -c cargo run --release
      devShells = forAllSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rust-analyzer
              pkg-config
              cmake
              gcc

              vulkan-loader
              vulkan-validation-layers
              vulkan-headers
              shaderc
              spirv-tools

              wayland
              wayland-protocols
              libxkbcommon

              libx11
              libxcursor
              libxi
              libxrandr
            ];

            VK_LAYER_PATH =
              "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
            SHADERC_LIB_DIR = "${pkgs.shaderc.lib}/lib";

            # Make the Vulkan loader and friends findable at runtime.
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.vulkan-loader
              pkgs.shaderc
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libx11
            ];

            shellHook = ''
              echo "hex-dodge dev shell — run: cargo run --release"
            '';
          };
        });
    };
}
