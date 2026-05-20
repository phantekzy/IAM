#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
#  build-linux.sh
#  Compiles the release binary and creates a .deb package
#  Requirements: Rust, dpkg-deb (pre-installed on Debian/Ubuntu/Fedora)
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

echo "=========================================================="
echo "  IAM Business - Linux Build Script"
echo "=========================================================="
echo ""

# Step 1 – compile
echo "[1/3] Compiling release binary..."
cargo build --release
echo "      Done."
echo ""

# Step 2 – prepare output dir
echo "[2/3] Preparing dist directory..."
mkdir -p dist/linux
echo "      Done."
echo ""

# Step 3 – create .deb
echo "[3/3] Building .deb package..."
chmod +x installer/linux/create-deb.sh
(cd installer/linux && bash create-deb.sh)
echo ""

echo "=========================================================="
echo "  BUILD COMPLETE"
echo ""
echo "  Binary  : target/release/iam-business"
echo "  Package : dist/linux/*.deb"
echo ""
echo "  Quick install:  sudo dpkg -i dist/linux/*.deb"
echo "  Quick remove :  sudo dpkg -r iam-business"
echo "=========================================================="
