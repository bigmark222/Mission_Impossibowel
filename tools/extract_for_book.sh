#!/usr/bin/env bash
set -euo pipefail

# Workspace root = repo root
ROOT=$(cd -- "$(dirname -- "$0")"/.. && pwd)
cd "$ROOT"

# List crates (workspace members with Cargo.toml)
crate_list() {
  echo "## Crates"
  rg --files -g 'Cargo.toml' | sed 's|^./||' | grep -v 'target/' | while read -r f; do
    dir="$(dirname "$f")"
    name="$(basename "$dir")"
    echo "- $name ($dir)"
  done
  echo
}

# Module tree per crate (depth 2)
module_tree() {
  crate_dir="$1"
  echo "### Modules for $crate_dir"
  (cd "$crate_dir" && find src -type f \( -name '*.rs' -o -name 'mod.rs' \) | sed 's|^|  - |')
  echo
}

# Pub items per crate
pub_items() {
  crate_dir="$1"
  echo "### Public API: $crate_dir"
  (cd "$crate_dir" && rg "^pub " -n src || true)
  echo
}

crate_list

# Iterate top-level crates (dirs containing Cargo.toml, excluding vendor/target)
find . -maxdepth 2 -name Cargo.toml \
  | grep -v './target/' \
  | grep -v './vendor/' \
  | while read -r manifest; do
      dir="$(dirname "$manifest")"
      echo "## Crate: $dir"
      module_tree "$dir"
      pub_items "$dir"
    done
