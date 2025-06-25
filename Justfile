dev:
    cargo build --lib
    cargo run --bin injector -- run ./target/i686-pc-windows-msvc/debug/epiphyte.dll

rel:
    cargo build --release --lib
    cargo run --bin injector -- run ./target/i686-pc-windows-msvc/release/epiphyte.dll