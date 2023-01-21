{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.66.0";
  wasmTarget = "wasm32-wasi";


  rustWithWasmTarget = rustPkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [ wasmTarget ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
  };

  common = {
    version = "0.1.21";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  disarray = rustPlatform.buildRustPackage (common // {
    pname = "disarray";

    buildPhase = ''
      cargo build --release --workspace
    '';  
    installPhase = ''
      mkdir -p $out/lib
      cp target/release/disarray $out/lib/
    '';  
  });
}