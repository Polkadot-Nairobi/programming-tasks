#!/bin/bash

set -e  # Exit on error

echo "[INFO] Step 1: Installing sccache..."
cargo install sccache
echo "[INFO] Step 1 completed."

echo "[INFO] Step 2: Configuring Cargo with sccache and mold..."
cat <<EOF > /usr/local/cargo/config.toml
[build]
rustc-wrapper = "/usr/local/cargo/bin/sccache"

[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
EOF
echo "[INFO] Cargo config written to /usr/local/cargo/config.toml"
echo "[INFO] Step 2 completed."

echo "[INFO] Step 3: Installing additional Rust tools (cargo-binstall, nu, bat)..."
if cargo install cargo-binstall ;
then
    cargo binstall bat nu -y
else
    cargo install bat nu
fi
echo "[INFO] Step 3 completed."


echo "[INFO] Step 4: Installing cargo contract (polka-vm)"
cargo install --locked --git https://github.com/use-ink/cargo-contract
echo "[INFO] Step 4 completed."

echo "[INFO] All steps completed successfully."
