{
  description = "A custom flake enabling consistent environments for Disarray";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        code = pkgs.callPackage ./. { inherit nixpkgs system rust-overlay; };
      in rec {
        packages = {
          workspace = code.workspace;
          all = pkgs.symlinkJoin {
            name = "all";
            paths = with code; [ ];
          };
          default = packages.all;
        };
      }
    );
}
