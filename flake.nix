{
  description = "Line Cook MCP Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        python = pkgs.python313;
        pythonPackages = python.pkgs;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            python
            uv
          ];
        };

        packages.default = pythonPackages.buildPythonPackage {
          pname = "line-cook";
          version = "0.1.0";
          src = ./.;

          # propagatedBuildInputs = with pythonPackages; [
          #   # Add MCP dependencies here
          # ];
        };
      }
    );
}

