{
  description = "Typst presentation for Rustikon 2026";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            typst
            tinymist
            typst
            typstyle
            iosevka
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
