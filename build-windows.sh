docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/odds-or-evens -w /usr/src/odds-or-evens rust:1.65.0 cargo build --release --target x86_64-pc-windows-gnu