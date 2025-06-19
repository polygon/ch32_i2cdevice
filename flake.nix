{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, rust-overlay, nixpkgs, naersk }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system;
        inherit overlays;
      };
      rust-bin = (pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          targets = [ ];
          extensions = [ "rust-src" ];
        }));
      rust-platform = pkgs.makeRustPlatform {
        cargo = rust-bin;
        rustc = rust-bin;
      };
      naersk-lib = naersk.lib.${system}.override {
        cargo = rust-bin;
        rustc = rust-bin;
      };
      deps = [
        pkgs.cargo-bloat
        pkgs.rust-analyzer
        pkgs.gdb
        pkgs.minicom
        #pkgs.flip-link
        #pkgs.probe-rs
        pkgs.rustfmt
        #pkgs.pkg-config
        #pkgs.openssl
        (pkgs.python3.withPackages (ps: [ ps.ipython ps.numpy ps.matplotlib ]))
      ];
    in {
      devShell.${system} = pkgs.mkShell { buildInputs = deps ++ [ rust-bin ]; };
    };
}
