{
  description = "Timetable optimizer helps you find the best course arrangement";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils"; # provides eachDefaultSystem
    rust-overlay.url = "github:oxalica/rust-overlay"; # provides common rust packages
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          # makes a `rust-bin` attribute available in nixpkgs
          (import inputs.rust-overlay)
        ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # cargo, clippy, cargo-fmt, rustdoc, rustfmt, and other tools
            (rust-bin.stable.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            cargo-watch
            cargo-edit
            trunk
            tailwindcss
          ];
        };
      }
    );
}
