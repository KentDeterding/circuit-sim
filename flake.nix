{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
	rust-overlay = {
		url = "github:oxalica/rust-overlay";
		inputs = {
			nixpkgs.follows = "nixpkgs";
			flake-utils.follows = "flake-utils";
		};
	};
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
    	(system:
			let
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs {
					inherit system overlays;
				};
			in
			with pkgs;
			{
				devShells.default = mkShell {
					nativeBuildInputs = with pkgs; [
						cargo-generate
						rust-bin.stable.latest.default
					];
					# buildInputs = [  ];

					shellHook = ''
						zsh
					'';
				};
			}
      );
}
