none:
  @just --list

xbuild:
  cargo build --target=armv7-unknown-linux-gnueabihf

release-cli:
  cargo build --release --example cli
  scp target/release/examples/cli canbox:/home/pi3

release-dc:
  cargo build --release --example dc
  scp target/release/examples/dc canbox:/home/pi3
