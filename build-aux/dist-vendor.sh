#!/usr/bin/env bash
set eux
cd "$MESON_SOURCE_ROOT"
mkdir "$MESON_DIST_ROOT"/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $MESON_DIST_ROOT/.cargo/config
mv vendor "$MESON_DIST_ROOT"
