#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
#  create-deb.sh  –  Builds a Debian / Ubuntu .deb installer
#  Run AFTER:  cargo build --release
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

APP=iam-business
VERSION=1.0.0
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")
PKG="${APP}_${VERSION}_${ARCH}"
ROOT="../../dist/linux/${PKG}"

echo "==> Building ${PKG}.deb"

# ── Binary must exist ─────────────────────────────────────────────────────────
BIN="../../target/release/${APP}"
if [ ! -f "$BIN" ]; then
  echo "ERROR: Binary not found at ${BIN}"
  echo "       Run 'cargo build --release' first."
  exit 1
fi

# ── Create directory tree ─────────────────────────────────────────────────────
mkdir -p "${ROOT}/DEBIAN"
mkdir -p "${ROOT}/usr/local/bin"
mkdir -p "${ROOT}/usr/share/applications"
mkdir -p "${ROOT}/usr/share/doc/${APP}"

# ── Copy files ────────────────────────────────────────────────────────────────
cp "$BIN" "${ROOT}/usr/local/bin/${APP}"
chmod 755 "${ROOT}/usr/local/bin/${APP}"
cp iam-business.desktop "${ROOT}/usr/share/applications/"
chmod 644 "${ROOT}/usr/share/applications/iam-business.desktop"

# ── Copyright ─────────────────────────────────────────────────────────────────
cat > "${ROOT}/usr/share/doc/${APP}/copyright" <<'COPY'
IAM Business Car Rental Manager
Copyright (C) 2026 IAM Business
All rights reserved.
COPY

# ── control file ──────────────────────────────────────────────────────────────
INSTALLED_SIZE=$(du -sk "${ROOT}" | cut -f1)
cat > "${ROOT}/DEBIAN/control" <<CTRL
Package: ${APP}
Version: ${VERSION}
Architecture: ${ARCH}
Maintainer: IAM Business <info@iam.business>
Installed-Size: ${INSTALLED_SIZE}
Depends: libxcb1, libxcb-render0, libxcb-shape0, libxcb-xfixes0
Section: misc
Priority: optional
Description: IAM Business Car Rental Manager
 Complete car rental management: fleet tracking, availability
 search, contracts, dashboard. Data is stored locally as CSV.
CTRL

# ── post-install: register .desktop ──────────────────────────────────────────
cat > "${ROOT}/DEBIAN/postinst" <<'POST'
#!/bin/sh
set -e
update-desktop-database /usr/share/applications || true
POST
chmod 755 "${ROOT}/DEBIAN/postinst"

# ── Build the package ─────────────────────────────────────────────────────────
mkdir -p "../../dist/linux"
dpkg-deb --build "${ROOT}" "../../dist/linux/${PKG}.deb"
echo ""
echo "✅  Package created: dist/linux/${PKG}.deb"
echo ""
echo "    Install with:  sudo dpkg -i dist/linux/${PKG}.deb"
echo "    Remove  with:  sudo dpkg -r ${APP}"
