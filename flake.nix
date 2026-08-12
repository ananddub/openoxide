{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs, system }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              clang
              llvmPackages.libclang
              openssl
              pkg-config
              atlas
              cargo-deny
              cargo-edit
              cargo-watch
              sqlx-cli
              rust-analyzer
              mold
              git
              curl
              gnutar
              gzip
              docker-client
              wireguard-tools
              iproute2
              dnsutils
              util-linux
              nodejs_22
              self.formatter.${system}
            ];

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };

            shellHook = ''
              export OPENOXIDE_TOOL_HOME="''${OPENOXIDE_TOOL_HOME:-$HOME/.cache/openoxide-tools}"
              mkdir -p "$OPENOXIDE_TOOL_HOME/bin"
              export PATH="$OPENOXIDE_TOOL_HOME/bin:/usr/local/bin:$PATH:$HOME/.cargo/bin"
              export BUILDKIT_HOST=docker-container://buildkit
              export DOCKER_API_VERSION=1.41

              # Ensure BuildKit daemon is running (required by railpack)
              if ! docker inspect buildkit > /dev/null 2>&1; then
                echo "Starting BuildKit daemon..."
                docker run --privileged -d --name buildkit --restart always moby/buildkit > /dev/null
              fi

              echo "OpenOxide dev shell ready."
              echo "Build tools: docker, pack/Paketo, nixpacks, railpack, heroku."
              echo "If any optional tool is missing, run: openoxide_ensure_build_tools"
            '';
          };
        }
      );

      formatter = forEachSupportedSystem ({ pkgs, ... }: pkgs.nixfmt);
    };

}
