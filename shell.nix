{
  mkShell,
  cargo,
  rust-analyzer,
  pkg-config,
  udev,
  callPackage,
  bad-apple-frames ? callPackage ./frames.nix {},
}:
mkShell {
  nativeBuildInputs = [
    cargo
    rust-analyzer
    pkg-config
  ];
  BAD_APPLE = "${bad-apple-frames}";
  buildInputs = [ udev ];
}
