{
	description = "Kato Kenya App";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
		};

	outputs = { self, nixpkgs, ... }:
		let
			system = "x86_64-linux";
			pkgs = import nixpkgs { inherit system; };
		in {
		 	packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
				pname = "Kato-Kenya-App";
				version = "0.1.0";
				src = ./.;
				cargoLock.lockFile = ./Cargo.lock;

				nativeBuildInputs = [ pkgs.makeWrapper ];
				buildInputs = [ pkgs.wayland pkgs.libxkbcommon pkgs.vulkan-loader pkgs.libGL pkgs.libglvnd ];

				postFixup = ''
					wrapProgram $out/bin/Kato-Kenya-App \
					--prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath [ pkgs.wayland pkgs.libxkbcommon pkgs.vulkan-loader pkgs.libGL pkgs.libglvnd ]}
					'';
				};
			};
}
		
