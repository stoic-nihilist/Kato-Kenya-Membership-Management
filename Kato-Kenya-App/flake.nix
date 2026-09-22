packages.default = pkgs.rustPlatform.buildPackage {
	pname = "Kato Kenya App";
	version = "0.1.0";
	src = ./.;
	cargoLock.lockFile = ./Cargo.lock
	};


