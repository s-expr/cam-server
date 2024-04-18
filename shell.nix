with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "rust-env";
  nativeBuildInputs = [
    rustc
    cargo
    pkg-config
    stdenv.cc.cc.lib
    llvmPackages.clang
    llvmPackages.libclang
    cmake
    rustPlatform.bindgenHook

  ];
  buildInputs = [
    binutils
    stdenv.cc.cc.lib
    (opencv.override  {
       enableGtk3 = true;
       enableUnfree = true;
    })
    llvmPackages.libclang
  ];
}
