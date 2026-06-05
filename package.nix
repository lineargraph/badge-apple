{
  pkg-config,
  makeWrapper,
  rustPlatform,
  udev,
  callPackage,
  bad-apple-frames ? callPackage ./frames.nix {},
}:

let metadata = builtins.fromTOML (builtins.readFile ./Cargo.toml); in
rustPlatform.buildRustPackage {
  src = ./.;
  pname = metadata.package.name;
  version = metadata.package.version;
  cargoHash =  "sha256-7g6Jnv/2ylKrngchAhebT/9J1WlDXkKLd3Vt6XNChq0=";
  buildInputs = [ udev ];
  nativeBuildInputs = [pkg-config makeWrapper];
  meta = {};
  postInstall = ''
    wrapProgram $out/bin/badge-apple \
      --set BAD_APPLE "${bad-apple-frames}"
  '';
}
