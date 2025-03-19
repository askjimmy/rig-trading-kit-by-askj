#!/bin/bash

set -e 

echo "Updating package lists..."
sudo apt update

echo "Installing node.js and yarn"
sudo apt install -y curl
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -  
sudo apt install -y nodejs
sudo npm install --global yarn
yarn --version

echo "Installing Git..."
sudo apt install -y git
git --version

echo "Installing GCC and build tools..."
sudo apt install -y build-essential clang

export CC=gcc
export CXX=g++

echo "Installing OpenSSL dependencies..."
sudo apt install -y pkg-config libssl-dev

echo "Installing Rust (as non-root)..."
if [ "$EUID" -eq 0 ]; then
    echo "Rust should NOT be installed as root! Please re-run this script as a normal user."
    exit 1
fi

export CARGO_DRIFT_FFI_STATIC=1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source "$HOME/.cargo/env"

echo "Rust installation complete."
rustc --version

echo "Downloading libdrift_ffi_sys..."
curl -L https://github.com/drift-labs/drift-ffi-sys/releases/download/v2.107.0/libdrift_ffi_sys.so -o libdrift_ffi_sys.so
sudo mv libdrift_ffi_sys.so /usr/lib/

echo "Setting up Drift FFI path..."
export CARGO_DRIFT_FFI_PATH=/usr/lib
echo 'export CARGO_DRIFT_FFI_PATH=/usr/lib' >> ~/.bashrc

echo "Installation complete! Restart your shell or run: source ~/.bashrc"
