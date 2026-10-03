{
  description = "Iris - Modern Wayland screenshot engine & GTK4 overlay canvas";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # 1. Configure Rust Toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };

        # 2. Build-time tools (compilers, code generators, build scripts)
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          wrapGAppsHook4 # Handles GTK schema/icon assets & environment variables
        ];

        # 3. Runtime shared libraries required by wayland, gtk4, and cairo bindings
        buildInputs = with pkgs; [
          adwaita-icon-theme
          librsvg
          libGL
          libgbm
          mesa
          libglvnd # Provides EGL / GL dispatch headers
          gtk4
          libadwaita
          gtk4-layer-shell
          wayland
          wayland-protocols
          cairo
          pango
          gdk-pixbuf
          glib
          wl-clipboard # Runtime utility for pipe/copy testing
        ];

        # 4. Cargo build options for workspace bin derivations
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        # --- Development Shell (`nix develop`) ---
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # Environment variables needed for local Rust builds and IDE completion
          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH"

            echo "   Rust toolchain: $(rustc --version)"
          '';
        };

        # --- Package Build Output (`nix build`) ---
        packages = {
          default = rustPlatform.buildRustPackage {
            pname = "iris";
            version = "0.1.0";
            src = ./.;

            # Update this hash once Cargo.lock exists (use pkgs.lib.fakeHash initially)
            cargoLock.lockFile = ./Cargo.lock;

            inherit nativeBuildInputs buildInputs;

            meta = with pkgs.lib; {
              description = "Modern Wayland screenshot utility & GTK4 interactive canvas";
              homepage = "https://github.com/your-username/iris";
              license = licenses.mit;
              platforms = platforms.linux;
            };
          };
        };
      }
    );
}
