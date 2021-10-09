with import <nixpkgs> {};
mkShell {
  name = "postgres-syntax";
  buildInputs = [
    cargo
    pkg-config
    libiconv
    libpg_query
  ];
  RUSTFLAGS = ["-L${pkgs.libpg_query.out}/lib"];
}
