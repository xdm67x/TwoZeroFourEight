#!/bin/bash
# =============================================================================
# Script de build et de développement pour TwoZeroFourEight (WASM)
#
# Pré-requis :
#   1. Installer Trunk : cargo install trunk
#   2. Ajouter la cible WASM : rustup target add wasm32-unknown-unknown
#
# Usage :
#   ./build.sh          → build + serveur de dev avec hot-reload (port 8080)
#   ./build.sh release  → build optimisé pour la production (dossier dist/)
# =============================================================================
set -e

if [ "$1" = "release" ]; then
    echo "🔨 Build production (wasm-release)..."
    trunk build --release --public-url /TwoZeroFourEight/
    echo "✅ Build terminé → dossier dist/"
else
    echo "🚀 Serveur de développement sur http://localhost:8080"
    trunk serve --port 8080
fi
