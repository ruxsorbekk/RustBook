{
pkgs ? let
  lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
  nixpkgs = fetchTarball {
    url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
    sha256 = lock.narHash;
  };
  in import nixpkgs { overlays = [];},
  ...
}: let
  # Manifest via Cargo.toml
  # manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in 
  pkgs.stdenv.mkDerivation {
    # name = "${manifest.name}-dev";
    name = "relm4-minimal";
    
    # Compile dependencies
    nativeBuildInputs = with pkgs; [
      # Hail the nix
      nixd
      statix
      deadnix
      alejandra
      
      # Rust
      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
      cargo-watch
      
      # Other compile time dependencies
      openssl
      # libressl
      
      # Gnome related
      gtk4
      ninja
      pango
      parted
      polkit
      gettext
      vte-gtk4
      pkg-config
      gdk-pixbuf
      libadwaita
      libgweather
      gnome-desktop
      appstream-glib
      wrapGAppsHook4
      desktop-file-utils
      gobject-introspection
      rustPlatform.bindgenHook
      
      # Bootstrap
      python3
    ];
    
    # Set environment variables
    RUST_BACKTRACE = "full";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }