{
  description = "Rust development template";

  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {inherit system;};
        toolchain = pkgs.rustPlatform;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (with toolchain; [
              cargo
              rustc
              rustLibSrc
            ])
            clippy
            rustfmt
            pkg-config
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
        };

        packages.default = self.packages.valhali;
        packages.valhalid = toolchain.buildRustPackage {
          pname = "valhalid";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-CZSu2oIqdUSMW5IfKqih7YFuFYU+UoETiwM4R4b6rEw=";
        };
        packages.valhali = toolchain.buildRustPackage {
          pname = "valhali";
          version = "0.1.0";
          src = ./.;
          cargoHash = "sha256-Bdiyo+3vQkQfPOqcooCuXo7zyKbgA/9kra5bNZPHLT4=";
        };

        apps.default = utils.lib.mkApp {
          drv = self.packages.default;
        };
      }
    )
    // {
      nixosModules.default = {
        config,
        lib,
        pkgs,
        ...
      }: let
        cfg = config.services.valhali;
      in {
        options.services.valhali = {
          enable = lib.mkEnableOption "Valhali";
          services = lib.mkOption {
            description = "Valhali service definitions";
            default = {};
            type = lib.types.attrsOf (lib.types.submodule ({...}: {
              options = {
                alias = lib.mkOption {
                  description = "Optional alias semantically associated with the service";
                  type = lib.types.nullOr lib.types.nonEmptyStr;
                  default = null;
                };

                kind = lib.mkOption {
                  description = "Service kind e.g. http";
                  type = lib.types.nonEmptyStr;
                };

                protocol = lib.mkOption {
                  description = "Underlying transport protocol";
                  type = lib.types.enum ["tcp" "udp"];
                  default = "tcp";
                };

                port = lib.mkOption {
                  description = "The port on which to advertise the service";
                  type = lib.types.port;
                };
              };
            }));
          };
          aliases = lib.mkOption {
            description = "Valhali alias definitions";
            default = [];
            type = lib.types.listOf lib.types.nonEmptyStr;
          };
        };

        config = lib.mkIf cfg.enable {
          environment.etc."valhali/config.toml".source = (pkgs.formats.toml {}).generate "config.toml" {
            inherit (cfg) aliases services;
          };

          services.avahi.enable = true;

          systemd.services.valhali = {
            description = "Valhali daemon";
            wantedBy = ["multi-user.target"];

            serviceConfig = {
              ExecStart = "${self.packages.${pkgs.system}.valhalid}/bin/valhalid /etc/valhali/config.toml";
            };
          };
        };
      };
    };
}
