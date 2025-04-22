class Movement < Formula
  desc "Movement CLI for interacting with the Movement network"
  homepage "https://github.com/movementlabsxyz/aptos-core"
  url "file://#{Dir.pwd}/movement-cli-v3.5.tar.gz"
  sha256 "0626826ffb341498fe6184de96d1a24f30d91f21904a4a105d527d85d874a7f9"
  version "0.0.1-alpha"

  depends_on "rust" => :build
  depends_on "cmake" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: "crates/aptos"), "--bin", "movement"
  end

  test do
    system "#{bin}/movement", "--help"
  end
end
