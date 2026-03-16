{
  description = "Typst presentation for Rustikon 2026";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # presentation
            typst
            tinymist
            typst
            typstyle
            iosevka

            # rust code
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

            # html stuff
            prettier
            nodePackages.vscode-langservers-extracted
          ];

          shellHook = ''
            export TYPST_FONT_PATHS="${pkgs.iosevka}/share/fonts"
            echo "Typst presentation environment ready"
            echo "Run 'typst compile main.typ' to build PDF"
            echo "Run 'typst watch main.typ' for live preview"
          '';
        };

        packages.default = pkgs.stdenv.mkDerivation {
          name = "rustikon-2026-slides";
          src = ./.;
          buildInputs = [pkgs.typst];
          buildPhase = ''
            typst compile main.typ slides.pdf
          '';
          installPhase = ''
            mkdir -p $out
            cp slides.pdf $out/
          '';
        };
      }
    );
}
