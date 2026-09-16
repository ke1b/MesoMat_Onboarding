{
  description = "Development Environment for Temperature and Humidity Sensor";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs = {self, nixpkgs}:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        
        nativeBuildInputs = with pkgs; [
          rustup
          probe-rs-tools
          pkg-config
          just
          minicom
        ];


        buildInputs = with pkgs; [
          libusb1
          udev
        ];

      shellHook = ''
        echo "Temperature and Humidity Sensor"
      '';
    
      };
    };
}
