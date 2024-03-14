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
    (opencv.override  {
       enableGtk3 = true;
       enableUnfree = true;
    })
    cmake
  ];
  buildInputs = [
    binutils
    stdenv.cc.cc.lib
    opencv
    llvmPackages.libclang
  ];
}
